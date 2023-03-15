// @generated automatically by Diesel CLI.

diesel::table! {
    agents (id) {
        id -> Int4,
        uuid -> Varchar,
        description -> Varchar,
        owner -> Int4,
    }
}

diesel::table! {
    permission_membership (id) {
        id -> Int4,
        permission -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    permissions (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    routers (id) {
        id -> Int4,
        agent -> Int4,
        snmp_community -> Nullable<Varchar>,
        ssh_username -> Nullable<Varchar>,
        ssh_password -> Nullable<Varchar>,
        conn_type -> Nullable<Varchar>,
        router_type -> Nullable<Varchar>,
    }
}

diesel::table! {
    tunnels (id) {
        id -> Int4,
        version -> Int4,
        router -> Int4,
        ip -> Varchar,
        dynamic_ip -> Bool,
        ip_class -> Int4,
        hostname -> Varchar,
        description -> Varchar,
        source -> Varchar,
        cost -> Int4,
        tunnel_type -> Varchar,
        topology_type -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(agents -> users (owner));
diesel::joinable!(permission_membership -> permissions (permission));
diesel::joinable!(permission_membership -> users (user_id));
diesel::joinable!(routers -> agents (agent));
diesel::joinable!(tunnels -> routers (router));

diesel::allow_tables_to_appear_in_same_query!(
    agents,
    permission_membership,
    permissions,
    routers,
    tunnels,
    users,
);
