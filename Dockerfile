from liuchong/rustup:stable-musl

WORKDIR /src
ADD . /src

RUN cargo test  -- --nocapture --test-threads=1