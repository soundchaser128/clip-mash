#!/bin/bash

cd backend && cargo +nightly fmt --check && cargo sqlx prepare --check && cargo nextest run && cd ..
cd frontend && npm run format-check && npm run lint && npm run test:once && cd ..
