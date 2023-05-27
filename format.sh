#!/bin/sh
cargo fix --allow-staged
cargo +nightly fmt
cd frontend
npm run format
cd ..
