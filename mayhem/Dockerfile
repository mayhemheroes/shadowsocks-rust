# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd crates/shadowsocks-service/fuzz && cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv crates/shadowsocks-service/fuzz/target/x86_64-unknown-linux-gnu/release/handshake_request /handshake_request && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd crates/shadowsocks-service/fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    mv crates/shadowsocks-service/fuzz/target/release/handshake_request /handshake_request_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /handshake_request /handshake_request_no_inst /
