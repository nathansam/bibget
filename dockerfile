FROM rust:1.87-alpine3.20 AS builder
WORKDIR /usr/src/bibget
COPY . .
RUN apk add --no-cache clang gcc lld musl-dev build-base zlib-dev krb5-dev openssl-dev openssl-libs-static
RUN cargo install --root /usr/local --path .

FROM alpine:latest
RUN apk add --no-cache openssl
COPY --from=builder /usr/local/bin/bibget /usr/local/bin/bibget
ENTRYPOINT ["bibget"]