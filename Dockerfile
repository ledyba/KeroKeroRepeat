FROM ekidd/rust-musl-builder as builder

WORKDIR /home/rust/src
COPY --chown=rust:rust . .

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch

WORKDIR /

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/KeroKeroRepeat /KeroKeroRepeat

EXPOSE 8080
ENTRYPOINT ["/KeroKeroRepeat"]
