# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest AS builder

# Add source code to the build stage.
ADD . /src
WORKDIR /src

# Compile the fuzzers.
RUN cd compiler/parser && cargo +nightly fuzz build --verbose

# Package Stage
FROM ubuntu:latest
COPY --from=builder /src/compiler/parser/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_* /fuzzers/