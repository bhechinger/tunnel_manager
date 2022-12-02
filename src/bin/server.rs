use std::env;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use tonic::transport::Server;

use tunnel_manager::api::*;
use tunnel_manager::handlers::*;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().try_init().unwrap();
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_max_conn_str = env::var("DB_MAX_CONNECTION").unwrap_or_else(|_| "5".to_string());
    let db_max_conn = db_max_conn_str.parse::<u32>().unwrap();
    let grpc_host = env::var("GRPC_HOST").unwrap_or_else(|_| "[::1]".to_string());
    let grpc_port = env::var("GRPC_PORT").unwrap_or_else(|_| "50051".to_string());

    let manager = ConnectionManager::<PgConnection>::new(db_url);

    // Create a connection pool
    let pool = Pool::builder()
        .test_on_check_out(true)
        .max_size(db_max_conn)
        .build(manager)
        .expect("Could not build connection pool");

    {
        // Run database migrations
        let conn = &mut pool.get().unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
    }

    let addr = format!("{}:{}", grpc_host, grpc_port).parse()?;

    // let agent = agents::AgentService::new(pool.clone());
    // let auth = login::AuthService::new(pool.clone());
    // let router = routers::RouterService::new(pool.clone());
    // let tunnel = tunnels::TunnelService::new(pool.clone());
    let user = users::UserService::new(pool.clone());
    let permission = permissions::PermissionService::new(pool.clone());

    println!("Running on port {}", grpc_port);

    Server::builder()
        // .add_service(agent_server::AgentServer::new(agent))
        // .add_service(auth_server::AuthServer::new(auth))
        // .add_service(router_server::RouterServer::new(router))
        // .add_service(tunnel_server::TunnelServer::new(tunnel))
        .add_service(user_server::UserServer::new(user))
        .add_service(permission_server::PermissionServer::new(permission))
        .serve(addr)
        .await?;

    Ok(())
}
