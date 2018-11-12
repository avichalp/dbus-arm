FROM rust:1.30.0

RUN apt-get update && apt-get install --yes dbus dbus-x11 pkg-config libdbus-1-dev vim

WORKDIR /usr/src/myapp

COPY . .

# RUN cargo install

ENV CARGO_HOME /usr/src/myapp/.cargo
