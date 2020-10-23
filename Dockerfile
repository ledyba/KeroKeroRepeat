FROM ekidd/rust-musl-builder

WORKDIR /home/rust/src
COPY --chown=rust:rust . .

RUN cargo build --release --target=x86_64-unknown-linux-musl && \
    cp /home/rust/src/target/x86_64-unknown-linux-musl/release/KeroKeroRepeat /KeroKeroRepeat
