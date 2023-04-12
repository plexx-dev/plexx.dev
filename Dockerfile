FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:buster-slim as runner
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/plexx-dev /usr/local/bin/plexx-dev
COPY . .

CMD ["plexx-dev"]