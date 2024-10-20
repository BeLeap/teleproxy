use clap::Args;

use crate::{client, proto};

#[derive(Args)]
pub struct ClientArgs {
    #[arg(short, long)]
    proxy_address: String,
    #[arg(short, long)]
    target_address: String,

    #[arg(short = 'k', long)]
    header_key: String,
    #[arg(short = 'v', long)]
    header_value: String,

    #[arg(short = 'a', long)]
    api_key: String,

    #[arg(short, long)]
    command: Option<String>,
}

pub fn handler(args: &ClientArgs) {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let args = args;
            let mut teleproxy_client =
                proto::teleproxy::teleproxy_client::TeleproxyClient::connect(
                    args.proxy_address.clone(),
                )
                .await
                .unwrap();

            match &args.command {
                None => {
                    let header_key = &args.header_key;
                    let header_value = &args.header_value;
                    let id = client::register(
                        &mut teleproxy_client,
                        args.api_key.to_string(),
                        header_key.to_string(),
                        header_value.to_string(),
                    )
                    .await
                    .unwrap();
                    tracing::info!(id, "client registered");

                    log::info!("starting listen with id: {}", id);
                    let _ = client::listen(
                        &mut teleproxy_client,
                        &args.api_key,
                        &id,
                        &args.target_address,
                    )
                    .await;

                    log::info!("clien deregister request with id: {}", id);
                    let _ = client::deregister(&mut teleproxy_client, args.api_key.to_string(), id)
                        .await;
                }
                Some(command) => match command.as_str() {
                    "dump" => {
                        let resp = teleproxy_client
                            .dump(tonic::Request::new(proto::teleproxy::DumpRequest {
                                api_key: args.api_key.to_string(),
                            }))
                            .await;

                        println!("{}", resp.unwrap().into_inner().dump);
                    }
                    "flush" => {
                        teleproxy_client
                            .flush(tonic::Request::new(proto::teleproxy::FlushRequest {
                                api_key: args.api_key.to_string(),
                            }))
                            .await
                            .unwrap();
                    }
                    _ => {
                        panic!("Unsupported command.");
                    }
                },
            }
        });
}
