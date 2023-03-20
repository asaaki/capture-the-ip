# syntax=docker/dockerfile:1-labs

ARG RUST_IMG_VER=1.68.0-bookworm
ARG BUSYBOX_IMG_VER=1.36.0-glibc
# note: do not use :nonroot tag, as it does not work with fly.io
ARG DISTROLESS_IMG_VER=cc

ARG MAGICPAK_VER=1.3.2
ARG MAGICPAK_ARCH=x86_64

ARG UPX_VER=4.0.2
ARG UPX_ARCH=amd64

FROM rust:${RUST_IMG_VER} as builder

ARG MAGICPAK_VER
ARG MAGICPAK_ARCH
ARG UPX_VER
ARG UPX_ARCH

ENV DEBIAN_FRONTEND noninteractive

# https://github.com/moby/buildkit/blob/master/frontend/dockerfile/docs/reference.md#example-cache-apt-packages
RUN rm -f /etc/apt/apt.conf.d/docker-clean; \
    echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

RUN \
  --mount=type=cache,target=/var/cache/apt,sharing=locked \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
    /bin/sh -c set -ex; \
    apt-get update && apt-get upgrade; \
    apt-get install -y ca-certificates clang cmake libssl-dev mold pkg-config

RUN update-ca-certificates --fresh

ADD https://github.com/coord-e/magicpak/releases/download/v${MAGICPAK_VER}/magicpak-${MAGICPAK_ARCH}-unknown-linux-musl /usr/bin/magicpak
RUN chmod +x /usr/bin/magicpak
RUN wget -O upx.tar.xz https://github.com/upx/upx/releases/download/v${UPX_VER}/upx-${UPX_VER}-${UPX_ARCH}_linux.tar.xz && \
    tar -xf upx.tar.xz --directory /usr/bin --strip-components=1 $(tar -tf upx.tar.xz | grep -E 'upx$')

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

RUN \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/app/target \
    cargo build --release -p cti_core && \
    cargo build --release --bins && \
    mkdir -p /app/bin && \
    cp /app/target/release/libcti_core.so /app/bin/ ; \
    cp /app/target/release/libcti_core.so /lib/ ; \
    cp /app/target/release/cti_* /app/bin/

# Note: Remove compression if you want to inspect linked shared libs;
# due to upx this gets hidden (the wrapper bin is static).
# In production image you can then run:
# /lib/x86_64-linux-gnu/ld-linux-x86-64.so.2 --list /app/bin/cti_server
RUN magicpak -v \
    --include /etc/passwd \
    --include /etc/group \
    --compress --upx-arg --best --upx-arg --lzma \
    /app/bin/cti_server /bundle.server
RUN magicpak -v \
    --compress --upx-arg --best --upx-arg --lzma \
    /app/bin/cti_refresher /bundle.refresher
RUN magicpak -v \
    --compress --upx-arg --best --upx-arg --lzma \
    /app/bin/cti_migrate /bundle.migrate
RUN mkdir -p /bundle && \
    cp -prnv \
        /bundle.server/* \
        /bundle.refresher/* \
        /bundle.migrate/* \
        /bundle/

### busybox ###

FROM busybox:${BUSYBOX_IMG_VER} as shell

WORKDIR /shell

RUN cd /shell; \
    cp /bin/busybox .; \
    for c in $(./busybox --list); do ln -s ./busybox ./$c; done

# ### prod image ### #

FROM gcr.io/distroless/${DISTROLESS_IMG_VER} as production

ENV PATH=/app/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

COPY --from=builder --chown=1001:1001 /bundle /.
COPY --from=builder /var/empty /var/empty
COPY --from=shell /shell /bin

USER 1001:1001

CMD ["/app/bin/cti_server"]
