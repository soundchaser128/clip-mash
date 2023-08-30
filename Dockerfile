FROM node:lts AS node
WORKDIR /app
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cd backend && cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/backend/recipe.json ./backend/recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cd backend && cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
COPY --from=node /app/dist ./frontend/dist
RUN cd backend && cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/backend/target/release/clip-mash /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
EXPOSE 5174
CMD ["/app/clip-mash", "0.0.0.0"]
