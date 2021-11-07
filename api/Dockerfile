FROM rustlang/rust:nightly

WORKDIR /usr/src/app

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo run

VOLUME ["/usr/local/cargo"]
