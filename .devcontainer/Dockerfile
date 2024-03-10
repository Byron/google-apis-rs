FROM python:3.8-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    wget \
    make \
    build-essential \
    curl \
    git \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && . $HOME/.cargo/env \
    && rustup default stable \
    && rustup update \
    && rustc --version \
    && rustup component add rustfmt \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set PATH environment variable to include the cargo bin directory
ENV PATH="/root/.cargo/bin:${PATH}"

# Run app.py when the container launches
CMD ["python", "app.py"]
