use api::agent_client::AgentClient;
use api::AgentGetRequest;
use uuid::Uuid;

pub mod api {
    tonic::include_proto!("api");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut client = AgentClient::connect("http://[::1]:50051").await?;
    // let id = Uuid::new_v4();

    // let list_response = client.list(()).await?;
    // println!("List RESPONSE={:?}", list_response);

    // let get_request = tonic::Request::new(AgentGetRequest {
    //     uuid: id.to_string(),
    // });

    // let get_response = client.get(get_request).await?;
    // println!("Get RESPONSE={:?}", get_response);

    Ok(())
}
