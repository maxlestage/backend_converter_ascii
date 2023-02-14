# FROM rust:1.64-slim-buster as build
# # RUN useradd -ms /bin/bash --system apiuser
# RUN cargo new --bin tp_wik_dps_tp02
# LABEL org.opencontainers.image.authors="maxime.lestage@ynov.com"
# WORKDIR /tp_wik_dps_tp02
# COPY ./Cargo.lock ./Cargo.lock
# COPY ./Cargo.toml ./Cargo.toml
# RUN cargo build --release
# RUN rm src/*.rs
# COPY ./src ./src
# RUN rm ./target/release/deps/tp_wik_dps_tp01*
# RUN cargo build --release

# FROM debian:buster-slim
# RUN useradd -ms /bin/bash --system apiuser
# USER apiuser
# LABEL org.opencontainers.image.authors="maxime.lestage@ynov.com"
# WORKDIR /
# COPY --from=build /tp_wik_dps_tp02/target/release/tp_wik_dps_tp01 .
# CMD ./tp_wik_dps_tp01
# EXPOSE 9000
# ----------------------------------------------------------------------------

FROM rust:latest as build
RUN cargo new --bin projet
WORKDIR /projet
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src
RUN rm ./target/release/deps/backend_converter_ascii*
RUN cargo build --release



FROM debian:buster-slim
RUN useradd -ms /bin/bash 
WORKDIR /
COPY --from=build /projet/target/release/backend_converter_ascii .
CMD ./backend_converter_ascii
EXPOSE 9000