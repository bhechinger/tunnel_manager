PGSQL_CONTAINER=tunnel_manager_pgsql

test: test-up test-run test-down

test-up:
	docker run --name $(PGSQL_CONTAINER) -e POSTGRES_PASSWORD=mysecretpassword -d postgres
	cargo run --bin server > /dev/null 2>&1  &

test-down:
	-kill $(shell ps -ef | awk '/target\/debug\/server/ { print $$2;}') 2> /dev/null
	-docker stop $(PGSQL_CONTAINER) 2> /dev/null
	-docker rm $(PGSQL_CONTAINER) 2> /dev/null

test-run:
	-cargo test