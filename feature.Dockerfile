# syntax=docker/dockerfile:1
ARG FEATURE_IMAGE
ARG BASE_IMAGE



FROM ${BASE_IMAGE} as base_base
RUN --mount


FROM ${FEATURE_IMAGE} as feature_base


