#
# npx openapi-generator generate -i openapi/openapi.yaml -g go-server -o generated_server
# npx openapi-generator generate -i openapi/openapi.yaml -g go -o generated_client
#
build: gen
	statik -src=./openapi_ui
	cd bin/ && go build -v ../

# The `validate` target checks for errors and inconsistencies in
# our specification of an API. This target can check if we're
# referencing inexistent definitions and gives us hints to where
# to fix problems with our API in a static manner.
validate:
	npx openapi-generator validate -i openapi.yaml


# The `gen` target depends on the `validate` target as
# it will only succesfully generate the code if the specification
# is valid.
#
# Here we're specifying some flags:
# --target              the base directory for generating the files;
# --spec                path to the swagger specification;
# --exclude-main        generates only the library code and not a
#                       sample CLI application;
# --name                the name of the application.
gen: validate
	npx openapi-generator generate -i openapi.yaml -g go-server -o .
	npx openapi-generator generate -i openapi.yaml -g openapi -o openapiui


deploy: build
	sls deploy

clean:
	rm bin/apiserver


# just added `gen` and `validate`
.PHONY: install gen validate clean
