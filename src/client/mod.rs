use crate::proto;

type Client = proto::teleproxy::teleproxy_client::TeleproxyClient<tonic::transport::Channel>;

pub type ClientResult<T> = Result<T, ClientError>;
#[derive(Debug)]
pub enum ClientError {
    RegisterError,
    DeregisterError,
}

pub async fn register(
    client: &mut Client,
    api_key: String,
    header_key: String,
    header_value: String,
) -> ClientResult<String> {
    match client
        .register(tonic::Request::new(proto::teleproxy::RegisterRequest {
            api_key,
            header_key,
            header_value,
        }))
        .await
    {
        Ok(response) => {
            let response = response.into_inner();
            Ok(response.id)
        }
        Err(err) => {
            log::error!("failed to register: {:?}", err);
            Err(ClientError::RegisterError)
        }
    }
}

pub async fn deregister(client: &mut Client, api_key: String, id: String) -> ClientResult<()> {
    match client
        .deregister(tonic::Request::new(proto::teleproxy::DeregisterRequest {
            api_key,
            id,
        }))
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!("failed to deregister with err: {:?}", err);
            Err(ClientError::DeregisterError)
        }
    }
}

pub async fn listen(client: &mut Client, api_key: &String, target: &String) -> ClientResult<()> {
    unimplemented!()
}
