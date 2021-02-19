FROM rust:1.50.0 as develop-stage
RUN cargo install sccache
ENV HOME=/server
ENV SCCACHE_CACHE_SIZE="1G"
ENV SCCACHE_DIR=$HOME/.cache/sccache
ENV RUSTC_WRAPPER="/usr/local/cargo/bin/sccache"

WORKDIR $HOME
RUN --mount=type=cache,target=/server/.cache/sccache cargo install cargo-watch

COPY . .

FROM develop-stage as build-stage
RUN --mount=type=cache,target=/server/.cache/sccache cargo build --release

FROM rust:1.50.0-slim
ENV DATABASE_URL="/data/people.db"
RUN apt update\
&& apt install -y sqlite3 libsqlite3-dev
EXPOSE "8080"
COPY --from=build-stage /server/target/release/server .
COPY ./people.db /data/
CMD ["/server"]
