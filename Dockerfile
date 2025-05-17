FROM rust:1.87 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM rust:1.87 as builder-wasm
WORKDIR /app/static/conway
COPY ./static/conway .
RUN cargo install wasm-pack
RUN wasm-pack build --target web


FROM debian:stable-slim as runner
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/plexx-dev /usr/local/bin/plexx-dev
COPY . .
COPY --from=builder-wasm /app/static/conway /app/static/conway
RUN rm -rf /app/static/conway/target

CMD ["plexx-dev"]