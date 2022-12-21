# syntax=docker/dockerfile:1-labs

ARG USER_NAME=appuser
ARG USER_ID=10001

ARG RUST_IMG_VER=1.65.0-bullseye
ARG BUSYBOX_IMG_VER=1.35.0-glibc
ARG DISTROLESS_IMG_VER=cc-debian11

ARG MOLD_VER=v1.7.1

ARG MAGICPAK_VER=1.3.2
ARG MAGICPAK_ARCH=x86_64

ARG UPX_VER=4.0.1
ARG UPX_ARCH=amd64

FROM rust:${RUST_IMG_VER} as builder

ARG MOLD_VER
ARG MAGICPAK_VER
ARG MAGICPAK_ARCH
ARG UPX_VER
ARG UPX_ARCH

ARG USER_NAME
ARG USER_ID

# https://github.com/moby/buildkit/blob/master/frontend/dockerfile/docs/reference.md#example-cache-apt-packages
RUN rm -f /etc/apt/apt.conf.d/docker-clean; \
    echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

RUN \
  --mount=type=cache,target=/var/cache/apt,sharing=locked \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
    /bin/sh -c set -ex; \
    apt-get update && apt-get upgrade; \
    apt-get install -y clang cmake pkg-config libssl-dev ca-certificates

RUN update-ca-certificates --fresh

# Note: if Rust's debian release gets updated to bookworm, we can apt install it
RUN git clone --depth 1 --branch ${MOLD_VER} https://github.com/rui314/mold.git; \
    mkdir mold/build; \
    cd mold/build; \
    ../install-build-deps.sh; \
    cmake \
        -DCMAKE_BUILD_TYPE=Release \
        -DMOLD_LTO=ON ..; \
    cmake --build . -j $(nproc); \
    cmake --install .

ADD https://github.com/coord-e/magicpak/releases/download/v${MAGICPAK_VER}/magicpak-${MAGICPAK_ARCH}-unknown-linux-musl /usr/bin/magicpak
RUN chmod +x /usr/bin/magicpak
RUN wget -O upx.tar.xz https://github.com/upx/upx/releases/download/v${UPX_VER}/upx-${UPX_VER}-${UPX_ARCH}_linux.tar.xz && \
    tar -xf upx.tar.xz --directory /usr/bin --strip-components=1 $(tar -tf upx.tar.xz | grep -E 'upx$')

ENV USER=${USER_NAME} \
    UID=${USER_ID}

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

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
# Inn production image you can then run:
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

COPY --from=builder --chown=appuser:appuser /bundle /.
COPY --from=builder /var/empty /var/empty
COPY --from=shell /shell /bin

USER appuser:appuser

CMD ["/app/bin/cti_server"]
