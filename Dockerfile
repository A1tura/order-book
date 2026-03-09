FROM rust:latest

WORKDIR /usr/src/app
COPY . .

WORKDIR /usr/src/app/service
RUN cargo install --path .

CMD ["service"]
