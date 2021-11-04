FROM rust:1.56.1

COPY . .

RUN cargo install --path .

CMD ["tcp-server"]