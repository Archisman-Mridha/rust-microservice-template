# syntax=docker/dockerfile:1

# STAGE 1 - BUILDING THE APPLICATION

ARG RUST_VERSION=1.70.0
ARG APP_NAME=authentication-microservice
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

COPY . .

RUN apt update -y && \
    apt install -y protobuf-compiler

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/ for downloaded dependencies and a cache mount
# to /app/target/ for compiled dependencies which will speed up subsequent builds.
# Once built, copy the executable to an output directory before the cache mounted /app/target is
# unmounted.
RUN --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/$APP_NAME /bin/server
EOF

# STAGE 2 - RUNNING THE APPLICATION

FROM debian:bullseye-slim AS final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#user
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
ARG PORT=4000
ARG PORT
EXPOSE ${PORT}

# What the container should run when it is started.
CMD ["/bin/server"]