#!/bin/sh
cargo fmt
cd frontend
npm run format
cd ..
