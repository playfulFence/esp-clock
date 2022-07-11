FROM  gitpod/workspace-base
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8

# ARGS
ARG CONTAINER_USER=gitpod
ARG CONTAINER_GROUP=gitpod
ARG TOOLCHAIN_VERSION=1.61.0.0
#ARG ESP_IDF_VERSION=release/v4.4
#ARG ESP_BOARD=esp32c3

# Install dependencies
RUN sudo install-packages git curl gcc ninja-build libudev-dev libpython2.7 \
    python3 python3-pip libusb-1.0-0 libssl-dev pkg-config libtinfo5 clang


# Set User
USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}

# Install Rust toolchain, extra crates and esp-idf
ARG INSTALL_RUST_TOOLCHAIN=install-rust-toolchain.sh
ENV PATH=${PATH}:/home/${CONTAINER_USER}/.cargo/bin:/home/${CONTAINER_USER}/opt/bin

# Use Rust and LLVM installer
ADD --chown=${CONTAINER_USER}:${CONTAINER_GROUP} \
    https://github.com/esp-rs/rust-build/releases/download/v${TOOLCHAIN_VERSION}/${INSTALL_RUST_TOOLCHAIN} \
    /home/${CONTAINER_USER}/${INSTALL_RUST_TOOLCHAIN}



RUN chmod a+x ${INSTALL_RUST_TOOLCHAIN} \
    && ./${INSTALL_RUST_TOOLCHAIN} \
    --extra-crates "cargo-espflash ldproxy" \
    --clear-cache "YES" --export-file /home/${CONTAINER_USER}/export-rust.sh \
    --esp-idf-version "release/v4.4"\
    --minified-esp-idf "YES" \
    --build-target "esp32c3"

# Install web-flash and wokwi-server
#RUN cargo install web-flash --git https://github.com/bjoernQ/esp-web-flash-server \
#    && RUSTFLAGS="--cfg tokio_unstable" cargo install wokwi-server --git https://github.com/MabezDev/wokwi-server --locked