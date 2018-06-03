## open-taffeta
Raspberry Pi-hosted website controlling access to a building

### Rust + application setup

Install Rust

`curl https://sh.rustup.rs -sSf | sh`

Install nightly build (Rocket does not yet run on stable):

`cargo toolchain install nightly`

Install sources (in case you want to use linting tools like `racer` or RLS):

`rustup component add rust-src`

### Additional tooling for this project

Install the ORM (Diesel) cli:

`cargo install diesel_cli`

Performs setup and install migrations:

`diesel setup`
`diesel migration generate create_users`

Run both migrations to check if they are ok:

`diesel migration redo`

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

### Check the SQLite file!

`$ sqlite3 app.db`

`sqlite> .tables;`

`sqlite> select * from users`
