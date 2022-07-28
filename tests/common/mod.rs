#![allow(dead_code)]

use std::{ffi::OsStr, panic, process, str, time::Duration};

use futures::Future;
use ocular::account::AccountInfo;
use tendermint_rpc::Client;

/// Chain ID to use for tests
pub const CHAIN_ID: &str = "tmingress-test";

/// Gas
pub const MULTISEND_BASE_GAS_APPROX: u64 = 60000;
pub const PAYMENT_GAS_APPROX: u64 = 25000;

/// ports
pub const INGRESS_RPC_PORT: u16 = 26655;
pub const RPC_PORT: u16 = 26657;
pub const GRPC_PORT: u16 = 9090;

/// Expected account numbers
// const SENDER_ACCOUNT_NUMBER: AccountNumber = 1;
// const RECIPIENT_ACCOUNT_NUMBER: AccountNumber = 9;

/// Bech32 prefix for an account
pub const ACCOUNT_PREFIX: &str = "cosmos";

/// Denom name
pub const DENOM: &str = "samoleans";

/// Example memo
// const MEMO: &str = "test memo";

// Gaia node test image built and uploaded from https://github.com/cosmos/relayer/tree/main/docker/gaiad;
// note some adaptations to file strucutre needed to build successfully (moving scripts and making directories)
// Disclaimer: Upon cosmos sdk (and other) updates, this image may need to be rebuilt and reuploaded.
pub const DOCKER_HUB_GAIA_SINGLE_NODE_TEST_IMAGE: &str = "philipjames11/gaia-test";

/// Execute a given `docker` command, returning what was written to stdout
/// if the command completed successfully.
///
/// Panics if the `docker` process exits with an error code.
pub fn exec_docker_command<A, S>(name: &str, args: A) -> String
where
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = process::Command::new("docker")
        .arg(name)
        .args(args)
        .stdout(process::Stdio::piped())
        .output()
        .unwrap_or_else(|err| panic!("error invoking `docker {}`: {}", name, err));

    if !output.status.success() {
        panic!("`docker {}` exited with error status: {:?}", name, output);
    }

    str::from_utf8(&output.stdout)
        .expect("UTF-8 error decoding docker output")
        .trim_end()
        .to_owned()
}

/// Invoke `docker run` with the given arguments.
pub fn docker_run<A, S>(args: A) -> String
where
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    exec_docker_command("run", args)
}

// Invoke `docker kill` with the given arguments.
pub fn docker_kill<A, S>(args: A) -> String
where
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    exec_docker_command("kill", args)
}

// Invoke `docker kill` then `docker rm` with the given arguments.
pub fn docker_cleanup(container_name: &str) -> String {
    let args = [container_name];
    docker_kill(args);

    let args = ["-f", container_name];
    exec_docker_command("rm", args)
}

pub fn init_tokio_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Could not build tokio runtime")
}

pub(crate) fn run_single_node_test<Fut>(container_name: &str, network_name: Option<&str>, test: fn(AccountInfo) -> Fut)
where
    Fut: Future<Output = ()>,
{
    let f = || init_tokio_runtime().block_on(async { surround(container_name, network_name, test).await });

    match panic::catch_unwind(f) {
        Ok(result) => result,
        Err(cause) => {
            // docker_cleanup(container_name);
            panic::resume_unwind(cause);
        }
    };
}

async fn surround<F, Fut>(container_name: &str, network_name: Option<&str>, test: F)
where
    F: FnOnce(AccountInfo) -> Fut,
    Fut: Future<Output = ()>,
{
    let sender_account = AccountInfo::new("");
    let sender_address = sender_account.address(ACCOUNT_PREFIX).unwrap();

    println!("Sender address: {}", sender_address);

    let rpc_binding = &format!("{}:{}", RPC_PORT, RPC_PORT);
    let grpc_binding = &format!("{}:{}", 9090, 9090);
    let docker_args = match network_name {
        Some(net_name) => vec![
            "-d",
            "-p",
            rpc_binding,
            "-p",
            grpc_binding,
            "--rm",
            "--name",
            container_name,
            "--net",
            net_name,
            DOCKER_HUB_GAIA_SINGLE_NODE_TEST_IMAGE,
            CHAIN_ID,
            &sender_address,
        ],
        None => vec![
            "-d",
            "-p",
            rpc_binding,
            "-p",
            grpc_binding,
            "--rm",
            "--name",
            container_name,
            DOCKER_HUB_GAIA_SINGLE_NODE_TEST_IMAGE,
            CHAIN_ID,
            &sender_address,
        ]
    };

    docker_run(&docker_args);
    test(sender_account).await;
    // docker_cleanup(container_name);
}

/// Wait for the node to produce the first block.
///
/// This should be used at the beginning of the test lifecycle to ensure
/// the node is fully booted.
pub async fn poll_for_first_block(rpc_client: &tendermint_rpc::HttpClient) {
    rpc_client
        .wait_until_healthy(Duration::from_secs(5))
        .await
        .expect("error waiting for RPC to return healthy responses");

    let mut attempts_remaining = 25;

    while let Err(e) = rpc_client.latest_block().await {
        if !matches!(e.detail(), tendermint_rpc::error::ErrorDetail::Serde(_)) {
            panic!("unexpected error waiting for first block: {:?}", e);
        }

        if attempts_remaining == 0 {
            panic!("timeout waiting for first block");
        }

        attempts_remaining -= 1;
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

pub fn generate_accounts(n: u64) -> Vec<AccountInfo> {
    let mut accounts = Vec::<AccountInfo>::new();

    for _ in 0..n {
        accounts.push(AccountInfo::new(""));
    }

    accounts
}
