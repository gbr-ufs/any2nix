FROM lukemathwalker/cargo-chef:0.1.78-rust-1.97.1@sha256:6dce65df3d7430c797e94348b4cf36d8d5876b63ca54f35dbfd37a97c42d0add AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --bin any2nix-api

FROM chef AS dev-builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --bin any2nix-api
COPY . .
RUN cargo build --bin any2nix-api

FROM chef AS prod-builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --bin any2nix-api
COPY . .
RUN cargo build --release --bin any2nix-api

FROM debian:bookworm-slim@sha256:88200866dfff7ea7f5cbcb6ec7c8a701889efe6fe859fe64d6990e4b07ea4171 AS dev
WORKDIR /app
COPY --from=dev-builder /app/target/debug/any2nix-api /usr/local/bin/any2nix-api
CMD ["/usr/local/bin/any2nix-api"]

FROM debian:bookworm-slim@sha256:88200866dfff7ea7f5cbcb6ec7c8a701889efe6fe859fe64d6990e4b07ea4171 AS prod
WORKDIR /app
COPY --from=prod-builder /app/target/release/any2nix-api /usr/local/bin/any2nix-api
CMD ["/usr/local/bin/any2nix-api"]
