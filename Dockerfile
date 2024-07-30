FROM rust:1.67-alpine AS builder
WORKDIR /usr/src/askeladd
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build --release

FROM alpine:3.14
RUN apk add --no-cache libgcc wget
COPY --from=builder /usr/src/askeladd/target/release/dvm_service_provider /usr/local/bin/dvm_service_provider
COPY --from=builder /usr/src/askeladd/target/release/dvm_customer /usr/local/bin/dvm_customer
COPY --from=builder /usr/src/askeladd/config /config
COPY --from=builder /usr/src/askeladd/.env.docker /.env
WORKDIR /usr/src/askeladd
ENV RUST_LOG=info
CMD ["dvm_service_provider"]