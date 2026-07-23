# syntax=docker/dockerfile:1

########################################
# Builder — compiles the blog-server release binary.
########################################
FROM rust:latest AS builder
WORKDIR /usr/src/app

# Which storage backend to compile in — matches blog-server's `postgres`/
# `sqlite` Cargo features. Defaults to postgres, matching docker-compose.yml's
# `db` service.
ARG DB_BACKEND=postgres

COPY . .
RUN cargo build --release --no-default-features --features ${DB_BACKEND} -p blog-server

########################################
# Runtime — a minimal image with just the compiled binary and bundled assets.
########################################
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/blog-server /usr/local/bin/blog-server
COPY --from=builder /usr/src/app/blog-views/templates ./templates

# blog-views embeds its templates path at compile time, valid only inside the
# builder stage; this override points it at where they actually live here.
ENV BLOG_VIEWS_ROOT=/app/templates

EXPOSE 8080

CMD ["blog-server"]
