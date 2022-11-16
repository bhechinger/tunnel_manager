extern crate dotenv;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tonic::transport::Server;
use tunnel_manager::api::*;
use tunnel_manager::handlers::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().try_init().unwrap();
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost/test".to_string());
    let db_max_conn = env::var("DB_MAX_CONNECTION").unwrap_or_else(|_| "5".to_string());
    let grpc_host = env::var("GRPC_HOST").unwrap_or_else(|_| "[::1]".to_string());
    let grpc_port = env::var("GRPC_PORT").unwrap_or_else(|_| "50051".to_string());

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(db_max_conn.parse().unwrap())
        .connect(&db_url)
        .await?;

    let addr = format!("{}:{}", grpc_host, grpc_port).parse()?;

    let agent = agents::AgentService::new(pool.clone());
    // let auth = login::AuthService::new(pool.clone());
    // let router = routers::RouterService::new(pool.clone());
    // let tunnel = tunnels::TunnelService::new(pool.clone());
    let user = users::UserService::new(pool.clone());
    let permission = permissions::PermissionService::new(pool.clone());

    println!("Running on port {}", grpc_port);

    Server::builder()
        .add_service(agent_server::AgentServer::new(agent))
        // .add_service(auth_server::AuthServer::new(auth))
        // .add_service(router_server::RouterServer::new(router))
        // .add_service(tunnel_server::TunnelServer::new(tunnel))
        .add_service(user_server::UserServer::new(user))
        .add_service(permission_server::PermissionServer::new(permission))
        .serve(addr)
        .await?;

    Ok(())
}
