FROM rust

WORKDIR /home

COPY . .

RUN rustup default nightly

RUN ./buildgamestatic.sh

RUN cargo build

CMD ROCKET_ENV=prod cargo run --release