FROM rust:1.89-bookworm AS builder

RUN apt-get update && apt-get install -y build-essential python3-dev

WORKDIR /simulator
COPY . .
RUN cargo install --path .
RUN rm -rf /simulator

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y python3-dev wget && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/distsys-gossip /usr/local/bin/distsys-gossip

WORKDIR /anysystem/python
RUN wget https://raw.githubusercontent.com/systems-group/anysystem/refs/heads/main/python/anysystem.py
ENV PYTHONPATH="/anysystem/python"

WORKDIR /impl
ENTRYPOINT ["distsys-gossip"]
