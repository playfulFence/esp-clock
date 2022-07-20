ARG VARIANT=bullseye
FROM debian:${VARIANT}
ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8

# Arguments
ARG CONTAINER_USER=esp
ARG CONTAINER_GROUP=esp
ARG TOOLCHAIN_VERSION=1.61.0.0
ARG ESP_IDF_VERSION=release/v4.4
ARG ESP_BOARD=esp32c3
ARG INSTALL_RUST_TOOLCHAIN=install-rust-toolchain.sh

# Install dependencies
RUN apt-get update \
    && apt-get install -y git curl gcc clang ninja-build libudev-dev unzip xz-utils\
    python3 python3-pip libusb-1.0-0 libssl-dev pkg-config libtinfo5 libpython2.7 \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/* /tmp/library-scripts

# Set users
RUN adduser --disabled-password --gecos "" ${CONTAINER_USER}
USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}

# Install toolchain with extra crates
ENV PATH=${PATH}:/home/${CONTAINER_USER}/.cargo/bin:/home/${CONTAINER_USER}/opt/bin

# Use Rust and LLVM installer
ADD --chown=${CONTAINER_USER}:${CONTAINER_GROUP} \
    https://github.com/esp-rs/rust-build/releases/download/v${TOOLCHAIN_VERSION}/${INSTALL_RUST_TOOLCHAIN} \
    /home/${CONTAINER_USER}/${INSTALL_RUST_TOOLCHAIN}

RUN chmod a+x ${INSTALL_RUST_TOOLCHAIN} \
    && ./${INSTALL_RUST_TOOLCHAIN} \
    --extra-crates "cargo-espflash ldproxy" \
    --clear-cache "YES" --export-file /home/${CONTAINER_USER}/export-rust.sh \
    --esp-idf-version "release/v4.4" \
    --minified-esp-idf "YES" \
    --build-target "esp32c3"

# Install web-flash and wokwi-server
RUN cargo install web-flash --git https://github.com/bjoernQ/esp-web-flash-server \
    && RUSTFLAGS="--cfg tokio_unstable" cargo install wokwi-server --git https://github.com/MabezDev/wokwi-server --locked

# Activate ESP environment
RUN echo "source /home/${CONTAINER_USER}/export-rust.sh" >> ~/.bashrc

ENV CURRENT_PROJECT=esp-clock
CMD [ "/bin/bash" ]
