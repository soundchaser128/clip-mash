#!/bin/bash

cd backend
cargo run &
cd ..
sleep 1
curl localhost:5174/api-docs/openapi.json > api-docs.json
openapi-generator-cli generate -g typescript-fetch -i api-docs.json -o typescript-client
rm api-docs.json
kill %1

