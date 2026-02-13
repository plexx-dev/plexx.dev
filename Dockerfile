FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM rust:latest as builder-wasm-conway
WORKDIR /app/game_files/conway
COPY ./game_files/conway .
RUN cargo install wasm-pack
RUN wasm-pack build --target web

FROM rust:latest as builder-wasm-snake
WORKDIR /app/game_files/snake
COPY ./game_files/snake .
RUN cargo install wasm-pack
RUN wasm-pack build --target web



FROM debian:stable-slim as runner
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/plexx-dev /usr/local/bin/plexx-dev
COPY . .
COPY --from=builder-wasm-conway /app/game_files/conway /app/game_files/conway
RUN rm -rf /app/game_files/conway/target
COPY --from=builder-wasm-snake /app/game_files/snake /app/game_files/snake
RUN rm -rf /app/game_files/snake/target

CMD ["plexx-dev"]