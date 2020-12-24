FROM ubuntu:bionic
ENV TERM=linux
ENV DEBIAN_FRONTEND=noninteractive
ENV CARGO_HOME=/usr/local
ENV RUSTUP_HOME=/usr/local

RUN apt-get update && \
    apt-get --no-install-recommends install -y \
        curl ca-certificates \
        build-essential pkg-config \
        libusb-1.0-0 libusb-1.0-0-dev libftdi1 libftdi1-dev

RUN curl --output /usr/local/bin/rustup-init -sS \
    https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init

RUN chmod a+x /usr/local/bin/* && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/* /usr/share/doc/*

RUN rustup-init -y --profile minimal --no-modify-path \
    --target x86_64-unknown-linux-gnu thumbv7em-none-eabihf

RUN cargo install cargo-flash && \
    cargo install cargo-embed

WORKDIR /usr/local

RUN find -type f -exec chmod ugo+rw {} \; && \
    find -type d -exec chmod ugo+rwx {} \;

WORKDIR /usr/local/src

CMD ["bash"]