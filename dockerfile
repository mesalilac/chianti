FROM rust:latest AS rust_builder

WORKDIR /src
COPY . /src/

RUN cargo build --release

FROM node:20.19.3-slim AS node_builder

WORKDIR /src
COPY ./web .

RUN npm install
RUN npm run build

FROM debian:trixie-backports

WORKDIR /app

COPY --from=rust_builder /src/target/release/chianti /app/
COPY --from=node_builder /src/dist /app/dist

EXPOSE 8080
VOLUME /app/data

CMD ["./chianti"]
