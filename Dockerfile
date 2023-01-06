FROM rust:latest
WORKDIR /usr/app
COPY . .
RUN cargo build --release
EXPOSE 8000
CMD ["./target/release/url_shortner"]