use std::net::SocketAddr;

use abscissa_core::{Command, Runnable};
use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{prelude::*, rpc::relay_rpc};

#[derive(Command, Debug, Parser)]
pub struct StartCmd;

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {
        abscissa_tokio::run(&APP, async {
            let config = APP.config();
            let address: SocketAddr = config
                .rpc
                .address
                .parse()
                .expect(format!("failed to parse address {}", config.rpc.address).as_str());

            if let Err(err) = relay_rpc(&address).await {
                status_err!("server error: {}", err);
                std::process::exit(1)
            }
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}

// impl config::Override<CosmosTxEndpointConfig> for StartCmd {
//     // Process the given command line options, overriding settings from
//     // a configuration file using explicit flags taken from command-line
//     // arguments.
//     fn override_config(
//         &self,
//         mut config: CosmosTxEndpointConfig,
//     ) -> Result<CosmosTxEndpointConfig, FrameworkError> {
//         if !self.recipient.is_empty() {
//             config.hello.recipient = self.recipient.join(" ");
//         }

//         Ok(config)
//     }
// }
