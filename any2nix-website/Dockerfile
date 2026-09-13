# SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
#
# SPDX-License-Identifier: GPL-3.0-or-later

FROM oven/bun:1.3.13@sha256:bb35eafd10b2e969809384850ff0474ba36a491239d715864bc87787b4cdf0a4 AS bun

FROM lukemathwalker/cargo-chef:0.1.78-rust-1.97.1@sha256:6dce65df3d7430c797e94348b4cf36d8d5876b63ca54f35dbfd37a97c42d0add AS chef
COPY --from=bun /usr/local/bin/bun /usr/local/bin/bun
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --bin any2nix-website

FROM chef AS dev-builder
ARG CARGO_FEATURES="default"
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --bin any2nix-website --no-default-features --features "${CARGO_FEATURES}"
COPY . .
RUN cd any2nix-website && bun install --frozen-lockfile
RUN cargo build --bin any2nix-website --no-default-features --features "${CARGO_FEATURES}"

FROM chef AS prod-builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --bin any2nix-website
COPY . .
RUN cd any2nix-website && bun install --frozen-lockfile
RUN cargo build --release --bin any2nix-website

FROM gcr.io/distroless/cc-debian12:debug@sha256:bc3546a529388660b583cc85ca6559927652d54391dc8059419d7b4bf3921154 AS dev
WORKDIR /app
COPY --from=dev-builder /app/target/debug/any2nix-website /usr/local/bin/any2nix-website
COPY --from=dev-builder /app/any2nix-website/assets /app/any2nix-website/assets
COPY --from=dev-builder /app/any2nix-website/node_modules /app/any2nix-website/node_modules
ENTRYPOINT ["/usr/local/bin/any2nix-website"]

FROM gcr.io/distroless/cc-debian12@sha256:e5d81ddde149641e2a9ba55be4545bc125c67de07508b03ba4c22e6eb0ded5aa AS prod
WORKDIR /app
COPY --from=prod-builder /app/target/release/any2nix-website /usr/local/bin/any2nix-website
ENTRYPOINT ["/usr/local/bin/any2nix-website"]
