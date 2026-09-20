# SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
#
# SPDX-License-Identifier: GPL-3.0-or-later

FROM oven/bun:1.3.13@sha256:bb35eafd10b2e969809384850ff0474ba36a491239d715864bc87787b4cdf0a4 AS bun

FROM lukemathwalker/cargo-chef:0.1.78-rust-1.97.1@sha256:6dce65df3d7430c797e94348b4cf36d8d5876b63ca54f35dbfd37a97c42d0add AS chef
COPY --from=bun /usr/local/bin/bun /usr/local/bin/bun
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS dev-builder
ARG CRATE="any2nix-cli"
ARG CARGO_FEATURES="default"
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --package "${CRATE}" --no-default-features --features "${CARGO_FEATURES}"
COPY . .
RUN if [ "${CRATE}" = "any2nix-website" ]; then cd any2nix-website && bun run build; fi
RUN cargo build --package "${CRATE}" --no-default-features --features "${CARGO_FEATURES}" && \
    if [ -f "/app/target/debug/${CRATE}" ]; then \
      cp "/app/target/debug/${CRATE}" /app/app-bin; \
    elif [ -f "/app/target/debug/any2nix" ]; then \
      cp /app/target/debug/any2nix /app/app-bin; \
    fi
RUN mkdir -p /app/any2nix-website/dist

FROM chef AS prod-builder
ARG CRATE="any2nix-cli"
ARG CARGO_FEATURES="default"
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --package "${CRATE}" --no-default-features --features "${CARGO_FEATURES}"
COPY . .
RUN if [ "${CRATE}" = "any2nix-website" ]; then cd any2nix-website && bun run build; fi
RUN cargo build --release --package "${CRATE}" --no-default-features --features "${CARGO_FEATURES}" && \
    if [ -f "/app/target/release/${CRATE}" ]; then \
      cp "/app/target/release/${CRATE}" /app/app-bin; \
    elif [ -f "/app/target/release/any2nix" ]; then \
      cp /app/target/release/any2nix /app/app-bin; \
    fi

FROM gcr.io/distroless/cc-debian12:debug@sha256:bc3546a529388660b583cc85ca6559927652d54391dc8059419d7b4bf3921154 AS dev
WORKDIR /app
COPY --from=dev-builder /app/app-bin /usr/local/bin/app
COPY --from=dev-builder /app/any2nix-website/dist /app/any2nix-website/dist
ENTRYPOINT ["/usr/local/bin/app"]

FROM gcr.io/distroless/cc-debian12@sha256:e5d81ddde149641e2a9ba55be4545bc125c67de07508b03ba4c22e6eb0ded5aa AS prod
WORKDIR /app
COPY --from=prod-builder /app/app-bin /usr/local/bin/app
ENTRYPOINT ["/usr/local/bin/app"]
