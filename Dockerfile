FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

#* Stage responsible for generating a Cargo chef recipe.
FROM chef AS planner
COPY . .
# Generate the recipe.json file.
RUN cargo chef prepare --recipe-path recipe.json

#* Generate binary for our program.
FROM chef AS builder
# Build dependencies for our Rust project using the recipe.json
# file we generated in the previous stage.
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
# Build our application
COPY . .
RUN cargo build --release --bin rust-multistage-container-builds

#* Package the application binary in a lightweight distroless container.
FROM gcr.io/distroless/cc-debian11:latest AS packager
WORKDIR /
COPY --from=builder /app/target/release/rust-multistage-container-builds /usr/local/bin/rust-multistage-container-builds
EXPOSE 8000
CMD ["/rust-multistage-container-builds"]