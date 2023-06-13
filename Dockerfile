FROM rust:1-buster AS builder
WORKDIR /usr/src/app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo install --path .

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN groupadd --gid 1000 artificial_friend && useradd --system --gid 1000 --no-create-home --shell /bin/false artificial_friend
USER artificial_friend

COPY --from=builder /usr/local/cargo/bin/artificial_friend /usr/local/bin/artificial_friend
CMD ["artificial_friend"]
