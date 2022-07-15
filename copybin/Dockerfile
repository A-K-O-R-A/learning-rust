FROM rust:1.62-alpine3.16 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM alpine:3.16
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
CMD ["myapp"]