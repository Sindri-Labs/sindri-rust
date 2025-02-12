FROM rust:bookworm

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    default-jre \
    git \
    jq \
    nodejs \
    npm

# Source a local `.bashrc` file from the working directory if it exists.
RUN echo '[[ -f /workspace/.bashrc ]] && source /workspace/.bashrc' >> ~/.bashrc
