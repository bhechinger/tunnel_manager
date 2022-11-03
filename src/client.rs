use agent::agent_client::AgentClient;
use agent::AgentGetRequest;

pub mod agent {
    tonic::include_proto!("agent");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AgentClient::connect("http://[::1]:50051").await?;

    // let request = tonic::Request::new(HelloRequest {
    //     name: "Tonic".into(),
    // });

    let response = client.list().await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
