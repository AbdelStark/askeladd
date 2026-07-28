# Build stage: compile the workspace with the pinned toolchain.
FROM rust:1.77-alpine AS builder
WORKDIR /usr/src/askeladd
RUN apk add --no-cache musl-dev
COPY . .
# rust-toolchain.toml pins nightly-2024-01-04; install it explicitly.
RUN rustup toolchain install nightly-2024-01-04 \
    && cargo build --release

# Runtime stage: minimal image with the two binaries.
FROM alpine:3.20
RUN apk add --no-cache libgcc wget
COPY --from=builder /usr/src/askeladd/target/release/dvm_service_provider /usr/local/bin/dvm_service_provider
COPY --from=builder /usr/src/askeladd/target/release/dvm_customer /usr/local/bin/dvm_customer
COPY --from=builder /usr/src/askeladd/config /usr/src/askeladd/config
WORKDIR /usr/src/askeladd
ENV RUST_LOG=info
CMD ["dvm_service_provider"]
