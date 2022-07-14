FROM rust:1.62.0-slim-buster as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian11
COPY --from=builder /app/target/release/viewer /
USER 65532:65532
CMD ["./viewer"]
