FROM liuchong/rustup:nightly

RUN apt-get update && \
        apt-get install -y libpq-dev libsqlite3-dev
RUN apt-get -yyq autoremove && \
        apt-get clean -yyq && \
        rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

ENV DATABASE_URL=app.db
ENV ROCKET_ENV=prod
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

ADD src /app/src
WORKDIR /app

RUN rustup default nightly
# maybe also chrono and r2d2?
RUN cargo install diesel_cli --force --no-default-features --features sqlite
RUN cargo install cargo-watch

COPY Cargo.toml ./Cargo.toml
COPY migrations ./migrations
COPY .env_prod .

RUN cargo build --release
RUN diesel migration run

EXPOSE 8080
