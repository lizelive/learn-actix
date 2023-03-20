# FROM mcr.microsoft.com/devcontainers/base:jammy
FROM ubuntu as apt-list
RUN rm -f /etc/apt/apt.conf.d/docker-clean && apt-get update
# install systemctl with cache

FROM ubuntu:jammy
RUN --mount=type=bind,source=apt-list,target=/var/lib/apt/lists \
    \ # --mount=type=cache,target=/var/lib/apt/lists,sharing=locked \
    --mount=type=cache,target=/var/cache/ \
    export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        systemctl
