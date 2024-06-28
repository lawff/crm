use crm::pb::{user_service_client::UserServiceClient, CreateUserRequest};
use tonic::Request;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = UserServiceClient::connect("http://[::1]:50051").await?;

    let request = Request::new(CreateUserRequest {
        name: "Alice".into(),
        email: "alice@gmail.com".into(),
    });

    let response = client.create_user(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
