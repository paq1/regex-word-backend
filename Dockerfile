FROM rust:latest
COPY . .
WORKDIR /
RUN cargo build --release
EXPOSE 8080
CMD ["./target/release/regex-word-backend"]