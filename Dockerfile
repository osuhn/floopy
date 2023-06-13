FROM rustlang/rust:nightly as builder
ENV RUSTFLAGS="-C target-cpu=skylake"

WORKDIR /floopy

RUN apt-get update && apt-get install -y cmake libtool libssl-dev && apt-get clean

# This is a dummy build to get the dependencies cached.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "// dummy file" > src/lib.rs && \
    cargo build --release && \
    rm -r src

# This is the actual build, copy in the rest of the sources
COPY . .
RUN cargo build --release --locked

# Now make the runtime container
FROM debian:bullseye-slim

RUN apt-get update && apt-get upgrade -y && apt-get install -y ca-certificates python3-pip ffmpeg && pip install -U yt-dlp && rm -rf /var/lib/apt/lists/*

COPY --from=builder /floopy/target/release/floopy /usr/local/bin/floopy
COPY Cargo.lock /

CMD ["/usr/local/bin/floopy"]
