FROM alpine:edge
WORKDIR /projects/consensus
RUN apk --update --no-cache add rust cargo protobuf zeromq-dev openssl-dev
ENV USER=root
RUN cargo init --bin .
COPY ./Cargo.* ./
RUN cargo build
COPY ./ ./
RUN cargo build
