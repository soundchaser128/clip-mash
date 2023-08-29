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
RUN cd /app/backend && cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/backend/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!

RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
COPY --from=node /app/dist /app/frontend/dist
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/clip-mash /app
EXPOSE 5173
RUN ls -la /app
CMD ["/app/clip-mash"]
