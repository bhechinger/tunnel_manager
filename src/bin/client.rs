use tonic::Request;

use api::user_client::UserClient;
use api::UserRequest;

use crate::api::user_request::IdOrEmail;

// use uuid::Uuid;

pub mod api {
    tonic::include_proto!("api");
}

fn get_token() -> &str {
    "Fake Token"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    let token = get_token(); // an method to get token can be a rpc call etc.
    let mut client = UserClient::with_interceptor(channel, move |mut req: Request<()>| {
        // adding token to request.
        req.metadata_mut().insert(
            "authorization",
            tonic::metadata::MetadataValue::from_bytes(&token).unwrap(),
        );
        Ok(req)
    });

    let list_response = client.list(()).await?;
    println!("List RESPONSE={:?}", list_response);

    let get_request = tonic::Request::new(UserRequest {
        id_or_email: Some(IdOrEmail::Id(1)),
    });

    let get_response = client.get(get_request).await?;
    println!("Get RESPONSE={:?}", get_response);

    Ok(())
}
