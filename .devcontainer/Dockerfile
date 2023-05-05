FROM --platform=linux/amd64 mcr.microsoft.com/devcontainers/rust:0-1-bullseye

# Install cang dependency
RUN apt update && export DEBIAN_FRONTEND=noninteractive && \
    apt install -y lsb-release wget software-properties-common gnupg && \
    rm -rf /var/lib/apt/lists/*

# Install Clang
RUN bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"