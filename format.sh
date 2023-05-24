#!/bin/sh
cargo fix --allow-staged
cargo fmt
cd frontend
npm run format
cd ..
