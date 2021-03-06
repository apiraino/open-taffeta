## open-taffeta [![Build Status](https://travis-ci.org/apiraino/open-taffeta.svg?branch=master)](https://travis-ci.org/apiraino/open-taffeta)
Raspberry Pi-hosted website controlling access to a building

The mobile App interfacing to this backend is [hosted here](https://github.com/apiraino/open-taffeta-mobile-app).

### Rust installation

Install Rust

`curl https://sh.rustup.rs -sSf | sh`

Select the custom setup and choose the `nightly` compiler (Rocket does not yet run on `stable`, see [Caveats](#caveats)).

If you have already Rust installed (`stable` or `beta`), simply install the additional toolchain:
``` bash
$ rustup toolchain install nightly
$ rustup override set nightly
```

Install sources (in case you want to use linting tools like `racer` or RLS):

`rustup component add rust-src`

### Additional libraries

Install `libssl-dev` to compile OpenSSL, `libmysqlclient-dev` to install `diesel-cli`.

### Additional tooling for this project

Install `cargo-edit` (to get `cargo add` superpowers) and `cargo-deb`.

Install the ORM (Diesel) cli:

`cargo install diesel_cli --no-default-features --features sqlite`

### Performs DB setup and migrations

``` bash
$ sh env.sh <ROCKET_ENV> (dev, staging, production)
$ diesel setup
$ diesel migration generate create_users
```

Write the SQL to create migrations (`up.sql` and `down.sql`)

Run both migrations to check if they are ok:

`diesel migration run`

### Running tests

Integration tests need the server running, so first launch `cargo run` in a shell and `cargo test --all` in another one.  Run one single test with ex. `cargo test test_list_users` or to run an entire directory of tests with ex. `cargo test --test test_users`.

To run test sequencially instead of parallelized (by default), use:

`RUST_TEST_THREADS=1 cargo test`

or

`cargo test -- --test-threads=1`

### Running the server

`cargo run`

with different logging enabled:

#### Logging levels:

Application and every (!) library (don't do this):
`RUST_LOG=debug cargo run`

Whole Open-Taffeta application:
`RUST_LOG=open_taffeta_lib=debug cargo run`

Whole Open-Taffeta and Rocket INFO:
`RUST_LOG=rocket::rocket=INFO,open_taffeta_lib=debug cargo run`

DB module only:
`RUST_LOG=open_taffeta_lib::db=debug cargo run`

### Deployment instructions

#### Install Rust nightly

``` bash
$ curl https://sh.rustup.rs -sSf | sh
$ rustup toolchain install nightly
$ rustup override set nightly
$ cargo install diesel_cli
```

### Get latest release package

- `https://github.com/apiraino/open-taffeta/releases/`
- `dpkg -i open-taffeta_x.y.z_amd64.deb`
- Configure `.env_dev` with buzzer endpoint
- `$ diesel setup`
- `sudo systemctl start open-taffeta`

### Testing the endpoint

`curl "http://localhost:8000" -H "Content-Type: application/json"`

The response should look like this JSON:

```
[{
    "email": "apiraino@users.noreply.github.com",
    "id": 1,
    "password": "123456",
    "username": "apiraino"
}, {
    "email": "kkom@users.noreply.github.com",
    "id": 2,
    "password": "654321",
    "username": "kkom"
}]
```

### As a Docker container

* `Dockerfile`: recipe to build the image

* `docker-build.sh`: script to rebuild the image (~1.7gb currently, working on making it thinner)

* `docker-run.sh`: script to tun locally the container for test

* `docker-compose -f docker-compose.yml up`: raise the container using docker compose (adviced)

### Endpoints

See [Wiki](https://github.com/apiraino/open-taffeta/wiki/Endpoints)

### Caveats

Keeping track of past and current issues, not related to the codebase but to external Rust tooling.

See [Wiki](https://github.com/apiraino/open-taffeta/wiki/Caveats).
