-- User table
CREATE TABLE users
(
    id    INTEGER PRIMARY KEY,
    email VARCHAR NOT NULL
);

ALTER TABLE public.users
    OWNER TO hecnet;

-- Permissions tables
CREATE TABLE permissions
(
    id          SERIAL PRIMARY KEY NOT NULL,
    name        VARCHAR(32)        NOT NULL,
    description VARCHAR(256)       NOT NULL
);

ALTER TABLE public.permissions
    OWNER TO hecnet;

CREATE TABLE permission_membership
(
    id         SERIAL PRIMARY KEY NOT NULL,
    permission INT REFERENCES permissions (id),
    user_id    INT REFERENCES users (id)
);

ALTER TABLE public.permission_membership
    OWNER TO hecnet;

-- Agent table
CREATE TABLE agents
(
    uuid  VARCHAR UNIQUE NOT NULL,
    owner INTEGER REFERENCES users (id)
);

ALTER TABLE public.agents
    OWNER TO hecnet;

-- Router table
CREATE TABLE routers
(
    id             SERIAL PRIMARY KEY NOT NULL,
    agent          VARCHAR REFERENCES agents (uuid),
    snmp_community VARCHAR,
    ssh_username   VARCHAR,
    ssh_password   VARCHAR,
    conn_type      VARCHAR,
    router_type    VARCHAR,
    CONSTRAINT "router_type can only be Cisco or PyDECNet" CHECK (router_type IN ('Cisco', 'PyDECNet')),
    CONSTRAINT "conn_type can only be SNMP or SSH" CHECK (conn_type IN ('SNMP', 'SSH'))
);

ALTER TABLE public.routers
    OWNER TO hecnet;

CREATE TABLE tunnels
(
    id            SERIAL PRIMARY KEY NOT NULL,
    version       INTEGER NOT NULL DEFAULT 0,
    router        INTEGER REFERENCES routers (id),
    ip            VARCHAR NOT NULL,
    dynamic_ip    BOOLEAN NOT NULL DEFAULT false,
    ip_class      INTEGER NOT NULL DEFAULT 4,
    hostname      VARCHAR NOT NULL,
    description   VARCHAR NOT NULL,
    source        VARCHAR NOT NULL,
    cost          INTEGER NOT NULL DEFAULT 10,
    tunnel_type   VARCHAR NOT NULL DEFAULT 'GRE',
    topology_type VARCHAR NOT NULL DEFAULT 'mesh',
    CONSTRAINT "id must be unique per router" UNIQUE (id, router),
    CONSTRAINT "ip_class can only be 4 or 6" CHECK (ip_class IN (4, 6)),
    CONSTRAINT "topology_type can only be mesh, hub or spoke" CHECK (topology_type IN ('mesh', 'hub', 'spoke')),
    CONSTRAINT "tunnel index (id) must be higher than 50" CHECK ((id >= 50)), --- make this configurable?
    CONSTRAINT "tunnel_type can only be GRE or IPSec" CHECK (tunnel_type IN ('GRE', 'IPSec'))
);

ALTER TABLE public.tunnels
    OWNER TO hecnet;
