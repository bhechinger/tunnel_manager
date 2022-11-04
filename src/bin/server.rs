// use futures_core::Stream;
// use std::pin::Pin;
// use std::sync::Arc;
// use tokio::sync::mpsc;
// use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tunnel_manager::api::agent_server::{AgentServer};

// use api::auth_server::{Auth, AuthServer};
// use api::router_server::{Router, RouterServer};
// use api::tunnel_server::{Tunnel, TunnelServer};
// use api::user_server::{User, UserServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let agent = tunnel_manager::handlers::agents::AgentService::default();
    // let auth = AuthService::default();
    // let router = RouterService::default();
    // let tunnel = TunnelService::default();
    // let user = UserService::default();

    Server::builder()
        .add_service(AgentServer::new(agent))
        // .add_service(AuthServer::new(auth))
        // .add_service(RouterServer::new(router))
        // .add_service(TunnelServer::new(tunnel))
        // .add_service(UserServer::new(user))
        .serve(addr)
        .await?;

    Ok(())
}
