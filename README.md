## open-taffeta
Raspberry Pi-hosted website controlling access to a building

### Rust + application setup

Install Rust

`curl https://sh.rustup.rs -sSf | sh`

Install nightly build (Rocket does not yet run on stable, see [Caveats](#caveats)):

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

### Endpoints

- [] `/`: welcome and stuff
- [] `/bell`: CRUD for bells (authenticated)
- [] `/admin`: manage users (authenticated)
- [x] `/users`: list of active guests (no auth, lol)
- [] `POST /opendoor?id=1`: open the door (authenticated)
  - request
``` json
{
        "user": "komo",
        "pass": 12345
}
```
  - response 201
```
{
        "msg": "opening door / doorbell not being rung"
}
```
  - response 401
```
{
        "msg": "auth failed"
}
```
- [] `POST /ring?id=1`: a doorbell is being rung
  - response 201
```
{
    "doorbell_id": "123123",
    "creation_ts": "1533849560"
}
```
  - save doorbell_id,timestamp

### Check the SQLite file!

`$ sqlite3 app.db`

`sqlite> .tables;`

`sqlite> select * from users`

### Caveats

`rocket` requires the rust `nightly` compiler (which is in flux by definition) and that gets a bit on the nerves to the `diesel` crate, so you may want to pin a working combo of a nightly version + `diesel` crate and carefully evaluate upgrades.

Q: *There's a warning when I derive Queryable (or other), but my code compiles*
```
5 | derive ( Debug , Clone , Copy , QueryId , Default ) ] pub struct $ column_name
  |                                 ^^^^^^^ names from parent modules are not accessible without an explicit import
```
A: Yeah, we know, nothing you can do atm, just ignore it or mute with:
```
RUSTFLAGS="-Aproc-macro-derive-resolution-fallback"
```
ref: [https://github.com/rust-lang/rust/issues/50504#issuecomment-409609119](https://github.com/rust-lang/rust/issues/50504#issuecomment-409609119)

Q: I want to test Rust 2018 preview, break all the things!

A: Sure, why not. Apply [these changes](https://www.ncameron.org/blog/how-to-help-test-the-2018-edition/) and run `cargo run`. If it won't compile, try using [cargo fix](https://rust-lang-nursery.github.io/edition-guide/editions/transitioning.html) to (hopefully) automagically fix your code.
