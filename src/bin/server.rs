use tonic::transport::Server;
use tunnel_manager::api::*;
use tunnel_manager::handlers::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let agent = agents::AgentService::default();
    let auth = login::AuthService::default();
    // let router = RouterService::default();
    // let tunnel = TunnelService::default();
    // let user = UserService::default();

    Server::builder()
        .add_service(agent_server::AgentServer::new(agent))
        .add_service(auth_server::AuthServer::new(auth))
        // .add_service(RouterServer::new(router))
        // .add_service(TunnelServer::new(tunnel))
        // .add_service(UserServer::new(user))
        .serve(addr)
        .await?;

    Ok(())
}
