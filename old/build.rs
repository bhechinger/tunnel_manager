fn main() {
    tonic_build::configure()
        .compile(
            &[
                "proto/api/agents.proto",
                "proto/api/login.proto",
                "proto/api/routers.proto",
                "proto/api/tunnels.proto",
                "proto/api/users.proto",
                "proto/api/permissions.proto",
            ],
            &["proto"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
