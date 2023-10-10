FROM rust:latest

WORKDIR /protocol

COPY . /protocol

# TODO: refactor containers' arguments, currently need to manually specify port exposed by container
ARG CONFIG_INDEX
ARG CONFIG_PATH
ARG BOOTSTRAPPING_TIME

# note: port must be manually exposed in docker-compose.yml based on config file

RUN cargo install --path .
RUN cargo build

CMD cargo run $CONFIG_PATH $CONFIG_INDEX $BOOTSTRAPPING_TIME