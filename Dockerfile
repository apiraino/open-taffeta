# Recipe for image for balena.io

FROM resin/raspberry-pi-debian:buster

ENV DATABASE_URL=app.db
ENV ROCKET_ENV=prod
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

ADD src /app/src
WORKDIR /app

# update distro to buster: fixes glibc mismatch with open-taffeta
RUN apt -y upgrade && apt -y update
RUN apt install -y build-essential libsqlite3-dev libssl-dev
RUN apt-get -yyq autoremove && \
        apt-get clean -yyq && \
        rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# install rust nightly
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
RUN . $HOME/.cargo/env
# maybe also chrono and r2d2?
RUN $HOME/.cargo/bin/cargo install diesel_cli --force --no-default-features --features sqlite
RUN $HOME/.cargo/bin/cargo install cargo-watch

COPY Cargo.toml ./Cargo.toml
COPY migrations ./migrations
COPY .env_prod .

RUN $HOME/.cargo/bin/cargo build --release
RUN diesel migration run

EXPOSE 8080
