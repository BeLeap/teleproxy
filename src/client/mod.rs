use crate::proto;

#[derive(Debug)]
pub enum ClientError {
    RegisterError,
}

pub async fn register(
    client: &mut proto::teleproxy::teleproxy_client::TeleproxyClient<tonic::transport::Channel>,
    api_key: &String,
    header_key: String,
    header_value: String,
) -> Result<String, ClientError> {
    match client.register(tonic::Request::new(proto::teleproxy::RegisterRequest {
        api_key: api_key.to_string(),
        header_key,
        header_value,
    })).await {
        Ok(response) => {
            let response = response.into_inner();
            Ok(response.id)
        },
        Err(err) => {
            log::error!("failed to register: {:?}", err);
            Err(ClientError::RegisterError)
        },
    }
}
