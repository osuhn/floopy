# Build image
# Necessary dependencies to build Parrot
FROM rust:slim-bullseye as build

RUN apt-get update && apt-get install -y \
    build-essential autoconf automake cmake libtool libssl-dev pkg-config

WORKDIR "/floopy"

# Cache cargo build dependencies by creating a dummy source
COPY src ./src
COPY Cargo.toml ./
COPY Cargo.lock ./
RUN cargo build --release --locked

# Release image
# Necessary dependencies to run Parrot
FROM debian:bullseye-slim as runner

RUN apt-get update && apt-get install -y python3-pip ffmpeg
RUN pip install -U yt-dlp

COPY --from=build /floopy/target/release/floopy .

CMD ["./floopy"]
