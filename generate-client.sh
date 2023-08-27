#!/bin/bash
set -e

cd backend
cargo build
cargo run &
cd ..
sleep 2
curl localhost:5174/api-docs/openapi.json > api-docs.json
openapi-generator-cli generate -g typescript-fetch -i api-docs.json -o typescript-client
rm api-docs.json
kill %1
