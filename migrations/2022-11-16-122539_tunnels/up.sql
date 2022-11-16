CREATE TABLE tunnels
(
    id            SERIAL PRIMARY KEY NOT NULL,
    version       INTEGER            NOT NULL DEFAULT 0,
    router        INTEGER REFERENCES routers (id),
    ip            VARCHAR            NOT NULL,
    dynamic_ip    BOOLEAN            NOT NULL DEFAULT false,
    ip_class      INTEGER            NOT NULL DEFAULT 4,
    hostname      VARCHAR            NOT NULL,
    description   VARCHAR            NOT NULL,
    source        VARCHAR            NOT NULL,
    cost          INTEGER            NOT NULL DEFAULT 10,
    tunnel_type   VARCHAR            NOT NULL DEFAULT 'GRE',
    topology_type VARCHAR            NOT NULL DEFAULT 'mesh',
    CONSTRAINT "id must be unique per router" UNIQUE (id, router),
    CONSTRAINT "ip_class can only be 4 or 6" CHECK (ip_class IN (4, 6)),
    CONSTRAINT "topology_type can only be mesh, hub or spoke" CHECK (topology_type IN ('mesh', 'hub', 'spoke')),
    CONSTRAINT "tunnel index (id) must be higher than 50" CHECK ((id >= 50)), --- make this configurable?
    CONSTRAINT "tunnel_type can only be GRE or IPSec" CHECK (tunnel_type IN ('GRE', 'IPSec'))
);
