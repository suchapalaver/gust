# We use the latest Rust stable release as base image
FROM rust:latest
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
apt update \
&& apt install -y --no-install-recommends \
default-libmysqlclient-dev
# Copy all files from our working environment to our Docker image
COPY . .
# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/grusterylist"]