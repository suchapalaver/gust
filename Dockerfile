FROM rust:latest

WORKDIR /app

RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
apt update \
&& apt install -y --no-install-recommends \
default-libmysqlclient-dev

COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/grusterylist"]