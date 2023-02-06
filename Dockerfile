FROM rust:latest as build

# create a new empty shell project
RUN USER=root cargo new --bin ssgithub
WORKDIR /ssgithub

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/ssgithub*
RUN cargo build --release

# our final base
FROM rust:latest

# copy the build artifact from the build stage
COPY --from=build /ssgithub/target/release/ssgithub .
# EXPOSE 80/tcp
EXPOSE 8888
# set the startup command to run your binary
CMD ["./ssgithub"]
