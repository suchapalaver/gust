FROM rust:latest

WORKDIR /app

RUN apt update && apt install -y --no-install-recommends \
  default-libmysqlclient-dev

COPY . .

RUN cargo build --release

RUN chmod +x target

ENTRYPOINT ["./target/release/gust"]
