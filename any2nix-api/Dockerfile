# SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
#
# SPDX-License-Identifier: GPL-3.0-or-later

FROM lukemathwalker/cargo-chef:0.1.78-rust-1.97.1@sha256:6dce65df3d7430c797e94348b4cf36d8d5876b63ca54f35dbfd37a97c42d0add AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --bin any2nix-api

FROM chef AS dev-builder
ARG CARGO_FEATURES="default"
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --bin any2nix-api --no-default-features --features "${CARGO_FEATURES}"
COPY . .
RUN cargo build --bin any2nix-api --no-default-features --features "${CARGO_FEATURES}"

FROM chef AS prod-builder
ARG CARGO_FEATURES="default"
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --bin any2nix-api
COPY . .
RUN cargo build --release --bin any2nix-api

FROM gcr.io/distroless/cc-debian12:debug@sha256:bc3546a529388660b583cc85ca6559927652d54391dc8059419d7b4bf3921154 AS dev
WORKDIR /app
COPY --from=dev-builder /app/target/debug/any2nix-api /usr/local/bin/any2nix-api
ENTRYPOINT ["/usr/local/bin/any2nix-api"]

FROM gcr.io/distroless/cc-debian12@sha256:e5d81ddde149641e2a9ba55be4545bc125c67de07508b03ba4c22e6eb0ded5aa AS prod
WORKDIR /app
COPY --from=prod-builder /app/target/release/any2nix-api /usr/local/bin/any2nix-api
ENTRYPOINT ["/usr/local/bin/any2nix-api"]
