use api::agent_server::{Agent, AgentServer};

pub mod api {
    tonic::include_proto!("api");
}

#[derive(Debug)]
struct APIService;
