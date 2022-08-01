use assay::assay;
use common::{
    exec_docker_command, poll_for_first_block, run_single_node_test, ACCOUNT_PREFIX, CHAIN_ID,
    DENOM, GRPC_PORT, INGRESS_RPC_PORT,
};
use ocular::{
    account::AccountInfo,
    chain::{
        client::{cache::Cache, ChainClient},
        config::ChainClientConfig,
    },
    keyring::Keyring,
    Coin,
};

use crate::common::{docker_run, RPC_PORT};

mod common;

#[assay]
fn happy_path() {
    // create docker network if it doesn't exist
    let network_name = "test-network";
    let args = vec!["create", network_name];
    if !exec_docker_command("network", vec!["ls"]).contains(network_name) {
        exec_docker_command("network", args);
    }

    // run tmingress container in the network
    let rpc_binding = &format!("{}:{}", INGRESS_RPC_PORT, INGRESS_RPC_PORT);
    let docker_args = vec![
        "-d",
        "-p",
        rpc_binding,
        "--rm",
        "--name",
        "tmingress",
        "--net",
        network_name,
        "tmingress:prebuilt",
    ];
    docker_run(docker_args);

    let container_name = "happy-path";

    run_single_node_test(
        container_name,
        Some(network_name),
        |sender_account: AccountInfo| async move {
            let mut chain_client = init_test_chain_client().await;

            // wait for the chain or you will pull your hair out over a race condition
            let temp_client =
                tendermint_rpc::HttpClient::new(format!("http://localhost:{}", RPC_PORT).as_str())
                    .unwrap();
            poll_for_first_block(&temp_client).await;

            let txm = chain_client.get_basic_tx_metadata().await.unwrap();
            let recipient = AccountInfo::new("");
            let response = chain_client
                .send(
                    &sender_account,
                    &recipient.address(ACCOUNT_PREFIX).unwrap(),
                    Coin {
                        amount: 100,
                        denom: DENOM.to_string(),
                    },
                    Some(txm),
                )
                .await
                .unwrap();

            assert!(response.check_tx.code.is_ok(), "CheckTx error");
            assert!(response.deliver_tx.code.is_ok(), "DeliverTx error");
        },
    );
}

async fn init_test_chain_client() -> ChainClient {
    let rpc_address = format!("http://localhost:{}", INGRESS_RPC_PORT);
    let rpc_client =
        tendermint_rpc::HttpClient::new(rpc_address.as_str()).expect("Could not create RPC");
    let grpc_address = format!("http://localhost:{}", GRPC_PORT);
    let mut cache = Cache::create_memory_cache(None, 10).unwrap();
    let _res = cache
        .grpc_endpoint_cache
        .add_item(grpc_address.clone(), 0)
        .unwrap();

    ChainClient {
        config: ChainClientConfig {
            chain_name: "cosmrs".to_string(),
            chain_id: CHAIN_ID.to_string(),
            rpc_address: rpc_address.clone(),
            grpc_address,
            account_prefix: ACCOUNT_PREFIX.to_string(),
            gas_adjustment: 1.2,
            default_fee: ocular::tx::Coin {
                amount: 0u64,
                denom: DENOM.to_string(),
            },
        },
        keyring: Keyring::new_file_store(None).expect("Could not create keyring."),
        rpc_client: rpc_client.clone(),
        cache: Some(cache),
        connection_retry_attempts: 0,
    }
}
