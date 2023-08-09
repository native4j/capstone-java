#!/bin/bash

sudo apt update -y

# Install dependencies
sudo apt install -y build-essential \
                    git \
                    wget \
                    cmake \
                    mingw-w64 \
                    gcc-arm-linux-gnueabihf \
                    gcc-aarch64-linux-gnu \
                    gcc-i686-linux-gnu \

# Install Rust targets
rustup target add x86_64-pc-windows-gnu
rustup target add i686-pc-windows-gnu

rustup target add x86_64-unknown-linux-gnu
rustup target add i686-unknown-linux-gnu

rustup target add aarch64-unknown-linux-gnu
rustup target add arm-unknown-linux-gnueabihf

rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

setup_osxcross() {
  # Set up OSX cross compilation toolchain
  git clone https://github.com/tpoechtrager/osxcross
  cd osxcross

  # Install osxcross dependencies
  ./tools/get_dependencies.sh

  # Download Big Sur 11.1 SDK
  wget -nc https://github.com/joseluisq/macosx-sdks/releases/download/11.1/MacOSX11.1.sdk.tar.xz
  mv MacOSX11.1.sdk.tar.xz tarballs/

  # Build clang
  UNATTENDED=yes OSX_VERSION_MIN=11.1 ./build.sh

  cd ../
}

if [ ! -d "osxcross/target" ]; then
  setup_osxcross
fi