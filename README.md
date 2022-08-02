# Cosmin

(Name pending)

A conceptual tx ingress and middleware for receiving/filtering/ordering/relaying transactions to Cosmos validator nodes.

## Getting Started

To build from source or just run the happy path test, you will need to have [`rustc` and `cargo` installed](https://www.rust-lang.org/learn/get-started).

Run the following this repo's root directory:

```bash
cargo build --release
```

The resulting binary can be found at `./target/release/cosmin` (or `./target/debug/cosmin` if built without the `--release` flag).

Start the application with the binary

```rust
cosmin --config config.toml start
```

or with `cargo`

```rust
cargo run -- --config config.toml start
```

## Happy Path

The `happy_path` test will demonstrate the process receiving and relaying a simple `MsgSend` transaction to a single-node test chain. It spins up two docker containers: one that runs the `cosmin` process on the host port 26655, and one that runs the chain. They are created in the same docker network so that they can communicate by container name (`cosmin` will send requests to http://happy-path:26657). The host machine acts as the client sending a transaction from within the test itself, which can be found in `./tests/happy_path.rs`.

For this test, the `cosmin` process must be started with a TOML configuration file with the follow values:

```toml
# The RPC endpoint exposed by the chain container
[node]
rpc = "http://happy-path:26657"

# The server address for the Cosmin RPC
[rpc]
address = "0.0.0.0:26655"
```

Save this in a file called `config.toml` in the root directory of the repo.

Next, build the docker containers and run the test with `make`:

```bash
make build
make test
```

The test makes two requests to the chain through the `cosmin` process. One to the `/status` endpoint to get the next sequence value of the sender's account, and one to the `/broadcast_tx_commit` endpoint to actually broadcast the transaction.


This application is authored using [Abscissa], a Rust application framework.

For more information, see:

[Documentation]

[Abscissa]: https://github.com/iqlusioninc/abscissa
[Documentation]: https://docs.rs/abscissa_core/

## Transaction Flow

                                                                                      _________________________________
                                                                                     |                                 |
 ___________                          __________________                        _____|_______________                  |
|           |-------TX-Request------>|                  |------TX-Request----->|                     |   Tendermint    |
|   Client  |                        |      Cosmin      |                      | Tendermint JSON-RPC |       /         |
|___________|<----Wrapped-Response---|__________________|<------Response-------|_____________________|  Cosmos Chain   |
                                                                                     |                                 |
                                                                                     |_________________________________|
