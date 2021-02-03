FROM rust

WORKDIR /home

COPY . .

RUN rustup default nightly

RUN ./buildgamestatic.sh

RUN cargo update
RUN cargo build --release

#ROCKET_ENV=prod

CMD cargo run --release