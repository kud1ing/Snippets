# First stage: compile.
# ======================================================================================================================
FROM rust:latest AS builder

COPY ./ .
RUN cargo build --release

# Second stage: copy only the files necessary for deployment to a new container image.
# ======================================================================================================================
FROM debian:bookworm-slim

COPY --from=builder target/release/rust_http_server_hyper /
USER root

ENV RUST_LOG=info
ENV RUST_BACKTRACE=full

EXPOSE 8000
CMD ["/rust_http_server_hyper"]
