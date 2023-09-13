#!/bin/bash
set -e

cd backend
cargo build
cargo run &
cd ..
sleep 2
curl localhost:5174/api-docs/openapi.json > api-docs.json
cd frontend && npm run generate && npm run format && cd ..
rm api-docs.json
kill %1
