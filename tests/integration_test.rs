use api::user_client::UserClient;

use crate::api::UserData;

pub mod api {
    tonic::include_proto!("api");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_api() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UserClient::connect("http://[::1]:50051").await?;

    let user1 = UserData {
        id: None,
        email: "test@example.com".to_string(),
    };

    let add_response = client.add(user1.clone()).await;
    assert!(!add_response.is_err());

    let add_response = client.add(user1).await;
    assert!(add_response.is_err());

    // let list_response = client.list(()).await?;
    // assert_eq!(list_response.into_inner().users[0].id, add_response.into_inner().id);

    Ok(())
}
