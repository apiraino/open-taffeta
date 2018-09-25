## open-taffeta
Raspberry Pi-hosted website controlling access to a building

### Rust + application setup

Install Rust

`curl https://sh.rustup.rs -sSf | sh`

Install a nightly build (Rocket does not yet run on stable, see [Caveats](#caveats)):
``` bash
$ rustup toolchain install nightly
$ rustup override set nightly
```

Install sources (in case you want to use linting tools like `racer` or RLS):

`rustup component add rust-src`

### Additional tooling for this project

Install the ORM (Diesel) cli:

`cargo install diesel_cli`

### Performs DB setup and migrations:

``` bash
$ sh env.sh <DEPLOY_MODE>
$ diesel setup
$ diesel migration generate create_users
```

Write the SQL to create migrations (`up.sql` and `down.sql`)

Run both migrations to check if they are ok:

`diesel migration run`

### Running tests:

Integration tests need the server running, so first launch `cargo run` in a shell and `cargo test` in another one.

### Running the server

`cargo run`

### Testing the endpoint

`curl "localhost:8000/user" -H "Content-Type: application/json"`

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

### Endpoints

See [Wiki](https://github.com/apiraino/open-taffeta/wiki/Endpoints)

### Check the SQLite file!

`$ sqlite3 app.db`

`sqlite> .tables`

`sqlite> select * from users;`

### Caveats

Keeping track of past and current issues, not related to the codebase but to external Rust tooling.

See [Wiki](https://github.com/apiraino/open-taffeta/wiki/Caveats).
