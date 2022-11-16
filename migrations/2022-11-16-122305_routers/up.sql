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
