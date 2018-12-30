# Recipe for image for balena.io

FROM resin/raspberry-pi-debian

ENV DATABASE_URL=app.db
ENV ROCKET_ENV=prod
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

ADD src /app/src
WORKDIR /app

# update distro to buster: fixes glibc mismatch with open-taffeta
RUN apt install -y software-properties-common
RUN add-apt-repository \
        "deb http://raspbian.raspberrypi.org/raspbian/ \
        buster main contrib non-free rpi"
RUN apt dist-upgrade && \
        apt -y upgrade
# RUN apt install -y libsqlite3-dev
RUN apt-get -yyq autoremove && \
        apt-get clean -yyq && \
        rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# install rust nightly
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y

# maybe also chrono and r2d2?
RUN cargo install diesel_cli --force --no-default-features --features sqlite
RUN cargo install cargo-watch

COPY Cargo.toml ./Cargo.toml
COPY migrations ./migrations
COPY .env_prod .

RUN cargo build --release
RUN diesel migration run

EXPOSE 8080
