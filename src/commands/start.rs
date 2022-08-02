use std::net::SocketAddr;

use abscissa_core::{Command, Runnable};
use clap::Parser;

use crate::{prelude::*, rpc::serve};

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
                .unwrap_or_else(|_| panic!("failed to parse address {}", config.rpc.address));

            // start the server
            if let Err(err) = serve(&address).await {
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
