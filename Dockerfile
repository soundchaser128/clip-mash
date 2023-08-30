FROM node:lts AS node
WORKDIR /app
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

FROM rust:latest AS builder
WORKDIR /app
COPY --from=node /app/dist ./frontend/dist
COPY . .
RUN cd backend && cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/backend/target/release/clip-mash /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
EXPOSE 5173
CMD ["/app/clip-mash"]
