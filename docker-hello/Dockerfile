FROM rust:latest
WORKDIR /home/miskat/hdd/CS/RustaceanUp/docker-hello/
COPY ./src ./src
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
CMD [ "cargo", "run" ]
