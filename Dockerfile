# syntax=docker/dockerfile:1-labs

FROM rust:1.74.1-bookworm as builder

ARG RUSTFLAGS

ENV DEBIAN_FRONTEND noninteractive

# https://github.com/moby/buildkit/blob/master/frontend/dockerfile/docs/reference.md#example-cache-apt-packages
RUN rm -f /etc/apt/apt.conf.d/docker-clean; \
    echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

RUN \
  --mount=type=cache,target=/var/cache/apt,sharing=locked \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
    /bin/sh -c set -ex; \
    apt-get update && apt-get upgrade; \
    apt-get install -y ca-certificates clang cmake libnss3 libnss3-dev libssl-dev mold pkg-config

RUN update-ca-certificates --fresh

COPY --link --from=ghcr.io/markentier/utilities:all-in-one /usr/bin/magicpak /usr/bin/magicpak
COPY --link --from=ghcr.io/markentier/utilities:all-in-one /usr/bin/upx /usr/bin/upx
COPY --link --from=ghcr.io/markentier/utilities:all-in-one /usr/bin/sccache /usr/bin/sccache

ENV SCCACHE_DIR=/tmp/sccache \
    SCCACHE_CACHE_SIZE=2G \
    SCCACHE_ERROR_LOG=/tmp/sccache.log

# # comment this if you want to not use sccache for building
# ENV CARGO_INCREMENTAL=0 \
#     RUST_LOG=sccache=info \
#     RUSTC_WRAPPER=/usr/bin/sccache

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 1001 \
    appuser

RUN mkdir -p /var/empty

WORKDIR /app

COPY . .

RUN rustc --version --verbose && \
    cargo --version --verbose

RUN echo "CURRENT RUSTFLAGS=${RUSTFLAGS}"

RUN \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/app/target \
    cargo build --release -p cti_core && \
    cargo build --release --bins && \
    mkdir -p /app/bin && \
    cp /app/target/release/libcti_core.so /lib/ ; \
    cp /app/target/release/cti_* /app/bin/

# Note: if you want to inspect linked shared libs;
# /lib/x86_64-linux-gnu/ld-linux-x86-64.so.2 --list /cti/cti_server
# https://github.com/coord-e/magicpak#note-on-name-resolution-and-glibc
RUN magicpak \
    $(find /app/bin -executable -type f) \
    /bundle \
    --install-to /cti/ \
    --include /etc/passwd \
    --include /etc/group \
    --include '/lib/x86_64-linux-gnu/libnss_*' \
    -v

# ### prod image ### #

# note: do not use :nonroot tag, as it does not work with fly.io
# FROM gcr.io/distroless/cc as production

# since we use MAGICPAK to collect all necessary runtime dependencies
# AND we also include a musl based shell (to be independent from glibc here),
# we can simply use scratch as our base instead of Google's distroless images
FROM scratch as production

LABEL service="cti_server"
LABEL tech.markentier.image.service="cti_server"
LABEL org.opencontainers.image.title="CTI - Capture The IP"
LABEL org.opencontainers.image.url="https://github.com/asaaki/capture-the-ip"
LABEL org.opencontainers.image.source="https://github.com/asaaki/capture-the-ip"

ENV PATH=/cti:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

COPY --from=builder --chown=1001:1001 /bundle /.
COPY --from=builder /var/empty /var/empty
COPY --link --from=ghcr.io/markentier/utilities:all-in-one /busybox /bin

COPY <<-"SCRIPT" /run.sh
	#!/bin/sh
	/cti/cti_server
SCRIPT
RUN chmod +x /run.sh

USER 1001:1001

CMD ["/run.sh"]
