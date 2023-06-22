# First stage: compile.
# ======================================================================================================================
FROM rust:latest AS builder

COPY ./ .
RUN cargo build --release

# Second stage: copy only the files necessary for deployment to a new container image.
# ======================================================================================================================
FROM debian:bookworm-slim

ARG APP_USER=appuser
ARG APP_GROUP=appuser

# Add an unprivileged user and group.
RUN groupadd ${APP_USER} && useradd -g ${APP_GROUP} ${APP_USER}

# Copy the artefacts from the first stage.
COPY --chown=${APP_USER}:${APP_GROUP} --from=builder target/release/rust_http_server_hyper /

# ----------------------------------------------------------------------------------------------------------------------
# Run the server.

USER ${APP_USER}
ENV RUST_LOG=info
ENV RUST_BACKTRACE=full

EXPOSE 8000
CMD ["/rust_http_server_hyper"]
