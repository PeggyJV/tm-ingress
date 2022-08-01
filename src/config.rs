use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct CosmosTxEndpointConfig {
    pub max_request_threads: usize,
    pub max_tx_batch_size: u64,
    pub request_timeout_secs: u64,
    pub node: NodeSection,
    pub rpc: RpcSection,
}

impl Default for CosmosTxEndpointConfig {
    fn default() -> Self {
        Self {
            max_request_threads: 4,
            max_tx_batch_size: 50,
            request_timeout_secs: 10,
            node: NodeSection::default(),
            rpc: RpcSection::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct NodeSection {
    pub rpc: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
///
pub struct RpcSection {
    pub address: String,
}

impl Default for RpcSection {
    fn default() -> Self {
        Self {
            address: String::from("127.0.0.1:26655"),
        }
    }
}
