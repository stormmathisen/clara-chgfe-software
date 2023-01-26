# syntax=docker/dockerfile:1
FROM rust:1.66

WORKDIR /usr/src/chgfe4
COPY . .

RUN cargo install --path .

CMD ["chgfe4"]
