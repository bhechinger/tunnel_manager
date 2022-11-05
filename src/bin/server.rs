use tonic::transport::Server;
use tunnel_manager::api::*;
use tunnel_manager::handlers::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let agent = agents::AgentService::default();
    let auth = login::AuthService::default();
    let router = routers::RouterService::default();
    let tunnel = tunnels::TunnelService::default();
    let user = users::UserService::default();

    Server::builder()
        .add_service(agent_server::AgentServer::new(agent))
        .add_service(auth_server::AuthServer::new(auth))
        .add_service(router_server::RouterServer::new(router))
        .add_service(tunnel_server::TunnelServer::new(tunnel))
        .add_service(user_server::UserServer::new(user))
        .serve(addr)
        .await?;

    Ok(())
}
