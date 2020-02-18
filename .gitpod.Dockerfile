FROM gitpod/workspace-full-vnc

USER gitpod

RUN sudo apt-get -q update && \
    sudo apt-get install -yq \
        qemu \
        qemu-efi \
        qemu-system-arm \
        qemu-system-x86 \
    && sudo rm -rf /var/lib/apt/lists/*

RUN bash -cl "rustup target add thumbv7em-none-eabihf rust-src llvm-tools-preview"

RUN bash -cl "cargo install "

RUN basu -cl "rustup toolchain install nightly && rustup default nightly"
