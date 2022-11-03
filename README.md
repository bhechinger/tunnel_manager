# HECnet Tunnel Manager

## History
The HECnet Tunnel Manager is a small tool I wrote maybe 6 or so years ago to assist with configuring multiple Cisco
routers. It sets up a mesh of GRE tunnels with DECnet routing enabled. It was a quick and dirty python script I wrote
in an afternoon. To its credit it is still running to this day without too many problems, however it is fragile and
hard to configure (configuring tunnels is literally hand writing SQL queries).

More about HECnet here: http://mim.stupi.net/hecnet.htm

The architecture is said python script using the `LISTEN/NOTIFY` mechanism of PostgreSQL to respond to a trigger. Once
triggered it builds a Cisco config snippet that is placed on a tftp server. It then uses SNMP to instruct the router to
fetch the config snippet and apply it.

This has several downsides.

The primary issue is in order to have this system configure your router you need to open
SNMP to the internet. Most people firewall it so that the host that runs the python script is the only one that can
talk to SNMP. That also has issues because I can't move to a new IP without orchestrating the change with everyone who
uses the service.

The second largest issue is it's also not really able to tell who needs to be updated so everyone gets their config
updated regardless of if they need it or not.

## HECnet Tunnel Manager "2.0"
(NOTE: I'm not actually going to call or version it as such. Just gotta get with the hip kids or whatever.)

The new architecture is going to be agent/server based. An agent that you run would be notified of changes and would
use the same SNMP/tftp/snippet process as before only limited to your local network. The IP address of the server would
be totally irrelevant.

## Components
### Tunnel Engine
Back end service that handles all configuration management and GRPC generated API service. Also running grpc-gateway
for RESTful access. Will push config changes to a RabbitMQ queue for agents to pick up.

### Tunnel Agent
Agent that configures routers. Will connect to a RabbitMQ queue and listen for instructions.
