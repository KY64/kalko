FROM rust:1.52.1-alpine3.12

WORKDIR /home

COPY . .

RUN cargo build --release \
    && mv target/release /usr/local/kalko \
    && ln -s /usr/local/kalko/kalko /usr/local/bin/kalko \
    && rm -rf *
