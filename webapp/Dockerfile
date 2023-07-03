######################
######  builder  #####
######################
FROM messense/rust-musl-cross:x86_64-musl as builder
WORKDIR /usr/src/webapp1
RUN apt-get update && apt-get install -y musl-tools musl-dev
RUN rustup target add x86_64-unknown-linux-musl
COPY config $HOME/.cargo/config.toml
COPY Cargo.toml Cargo.toml
# cache dependencies layer
RUN mkdir src &&  echo "fn main(){println!(\"if you see this infomation, means the build failed.\")}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl
# build real source
COPY ./src ./src
# update the file date & build it.
RUN touch ./src/main.rs && cargo build --release --target x86_64-unknown-linux-musl

######################
######  runner  ######
######################
#FROM alpine:3
FROM scratch
COPY --from=builder /usr/src/webapp1/target/x86_64-unknown-linux-musl/release/webapp1 /app/
#RUN echo "$(ls -l /app)"
EXPOSE 5000
ENTRYPOINT ["/app/webapp1"]