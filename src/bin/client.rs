use api::user_client::UserClient;
use api::UserRequest;

use crate::api::user_request::IdOrEmail;

// use uuid::Uuid;

pub mod api {
    tonic::include_proto!("api");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UserClient::connect("http://[::1]:50051").await?;

    let list_response = client.list(()).await?;
    println!("List RESPONSE={:?}", list_response);

    let get_request = tonic::Request::new(UserRequest {
        id_or_email: Some(IdOrEmail::Id(1)),
    });

    let get_response = client.get(get_request).await?;
    println!("Get RESPONSE={:?}", get_response);

    Ok(())
}
