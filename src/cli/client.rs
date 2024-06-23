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

            let header_key = &args.header_key;
            let header_value = &args.header_value;
            let id = client::register(
                &mut teleproxy_client,
                &args.api_key,
                header_key.to_string(),
                header_value.to_string(),
            )
            .await
            .unwrap();
            log::info!("clien registered with id: {}", id);

            let _ = client::deregister(&mut teleproxy_client, &args.api_key, &id).await;
            log::info!("clien deregistered with id: {}", id);
        });
}
