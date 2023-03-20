#!/bin/sh
cd $(dirname $0)
wget -ci source.txt


# docker run --rm \
#   -v ${PWD}:/local openapitools/openapi-generator-cli generate \
#   -i /local/petstore.yaml \
#   -g go \
#   -o /local/out/go