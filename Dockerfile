# https://github.com/LukeMathWalker/cargo-chef
FROM lukemathwalker/cargo-chef:latest-rust-slim-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin linker

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/linker /usr/local/bin
ENV DATABASE_URL="sqlite:/data/db.sqlite"
ENV PORT=8000
ENTRYPOINT ["/usr/local/bin/linker"]
EXPOSE 8000
