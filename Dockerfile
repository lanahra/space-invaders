# Download base image rust
FROM rust

# Create /usr/projects/space-invaders directory
RUN mkdir -p /usr/projects/space-invaders

# Copy project directory
COPY . /usr/projects/space-invaders

RUN cd /usr/projects/space-invaders && \
    cargo build