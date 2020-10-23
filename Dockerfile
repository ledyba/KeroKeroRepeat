FROM ekidd/rust-musl-builder as builder

WORKDIR /home/rust/src
COPY --chown=rust:rust . .

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch

WORKDIR /

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/kerokero_repeat kerokero_repeat

RUN ["chmod", "a+x", "/kerokero_repeat"]

EXPOSE 8080
ENTRYPOINT ["/kerokero_repeat"]
