#!/bin/sh
cd backend
cargo fix --allow-staged
cargo +nightly fmt
cd ../frontend
npm run format
cd ..
