# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest AS BUILDER

# Add source code to the build stage.
ADD . /src
WORKDIR /src

# Compile the fuzzers
RUN cd compiler && cargo +nightly fuzz build
RUN cd compiler/parser && cargo +nightly fuzz build

# Package stage
FROM ubuntu:latest AS PACKAGE

# Copy the corpora to the final image
COPY --from=BUILDER /src/compiler/fuzz/corpus /corpus/compiler
COPY --from=BUILDER /src/compiler/parser/fuzz/corpus /corpus/parser

# Copy the fuzzers to the final image
COPY --from=builder /src/compiler/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_* /fuzzers/
COPY --from=builder /src/compiler/parser/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_* /fuzzers/