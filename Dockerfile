FROM ubuntu:latest

WORKDIR /app

RUN apt update

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/rust_backend_template"]