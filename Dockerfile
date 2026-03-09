FROM rust:latest

WORKDIR /usr/src/caviar

# RUN apt-get update && apt-get install -y git
COPY . .
RUN mkdir -p results tmp
RUN cargo build --release
CMD ["/bin/bash"]