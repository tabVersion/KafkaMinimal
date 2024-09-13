FROM ubuntu:24.04 AS base

ENV LANG en_US.utf8

RUN apt-get update \
  && apt-get -y install ca-certificates build-essential libsasl2-dev openjdk-17-jdk software-properties-common python3.12 python3.12-dev openssl pkg-config curl


FROM base AS rust-base

RUN apt-get update && apt-get -y install make cmake protobuf-compiler bash lld unzip rsync

SHELL ["/bin/bash", "-c"]

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --no-modify-path --default-toolchain none -y
ENV PATH /root/.cargo/bin:$PATH
ENV CARGO_INCREMENTAL=0

COPY rust-toolchain rust-toolchain
RUN rustup self update \
  && rustup set profile minimal \
  && rustup show \
  && rustup component add rustfmt

FROM rust-base AS rust-builder

COPY ./ /kafka_minimal
WORKDIR /kafka_minimal
RUN cargo build --release

FROM rust-builder AS kafka-minimal

COPY --from=rust-builder /kafka_minimal/target/release/KafkaMinimal /KafkaMinimal
RUN rm -rf /kafka_minimal

ENTRYPOINT [ "/KafkaMinimal" ]
