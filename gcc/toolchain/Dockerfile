FROM ubuntu:focal
ENV TERM=linux
ENV DEBIAN_FRONTEND=noninteractive
WORKDIR /usr/local/src

RUN apt-get update && \
    apt-get -y --no-install-recommends install \
        binutils-arm-none-eabi \
        gcc-arm-none-eabi \
        make && \
    apt-get -y --purge autoremove && \
    apt-get -y upgrade && \
    apt-get clean
