FROM rust:bookworm


# Source a local `.bashrc` file from the working directory if it exists.
RUN echo '[[ -f /workspace/.bashrc ]] && source /workspace/.bashrc' >> ~/.bashrc
