# syntax=docker/dockerfile:1-labs

FROM rust:1.72.0-bookworm as builder

ARG MAGICPAK_VER=1.4.0
ARG MAGICPAK_ARCH=x86_64

ARG UPX_VER=4.1.0
ARG UPX_ARCH=amd64

ARG SCCACHE_VER=v0.5.4
ARG SCCACHE_ARCH=x86_64

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

ADD --link --chmod=0755 https://github.com/coord-e/magicpak/releases/download/v${MAGICPAK_VER}/magicpak-${MAGICPAK_ARCH}-unknown-linux-musl /usr/bin/magicpak

RUN wget -O upx.tar.xz https://github.com/upx/upx/releases/download/v${UPX_VER}/upx-${UPX_VER}-${UPX_ARCH}_linux.tar.xz && \
    tar -xf upx.tar.xz --directory /usr/bin --strip-components=1 $(tar -tf upx.tar.xz | grep -E 'upx$')

RUN wget -O sccache.tar.gz https://github.com/mozilla/sccache/releases/download/${SCCACHE_VER}/sccache-${SCCACHE_VER}-${SCCACHE_ARCH}-unknown-linux-musl.tar.gz && \
    tar -xf sccache.tar.gz --directory /usr/bin --strip-components=1 $(tar -tf sccache.tar.gz | grep -E 'sccache$')

ENV SCCACHE_DIR=/tmp/sccache \
    SCCACHE_CACHE_SIZE=2G \
    SCCACHE_ERROR_LOG=/tmp/sccache.log

# comment this if you want to not use sccache for building
ENV CARGO_INCREMENTAL=0 \
    RUST_LOG=sccache=info \
    RUSTC_WRAPPER=/usr/bin/sccache

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

RUN rustc --version --verbose && cargo --version --verbose

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

### busybox ###

FROM busybox:1.36.1-musl as shell

WORKDIR /shell

RUN cd /shell; \
    cp /bin/busybox .; \
    for c in $(./busybox --list); do ln -s ./busybox ./$c; done

# ### prod image ### #

# note: do not use :nonroot tag, as it does not work with fly.io
# FROM gcr.io/distroless/cc as production

# since we use MAGICPAK to collect all necessary runtime dependencies
# AND we also include a musl based shell (to be independent from glibc here),
# we can simply use scratch as our base instead of Google's distroless images
FROM scratch as production

ARG RUST_BACKTRACE

ENV PATH=/cti:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

COPY --from=builder --chown=1001:1001 /bundle /.
COPY --from=builder /var/empty /var/empty
COPY --from=shell /shell /bin

USER 1001:1001

CMD ["/cti/cti_server"]
