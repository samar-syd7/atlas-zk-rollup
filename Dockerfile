FROM rust:1.89

WORKDIR /app

COPY . .

RUN cargo build --release -p sequencer

EXPOSE 3000

CMD ["./target/release/sequencer"]