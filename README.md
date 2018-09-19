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

All responses are `application/json`.

- [x] `/`: welcome and stuff (response `text/html; charset=utf-8`)
- [x] `/signup`: user signup
  - request
```
{
        "username": "john",
        "email": "hey@email.com",
        "password": "123456"
}
```
  - will create user with `active=true`
  - response 201
```
{
        "user": user_data
}
```
  - response 400
```
{
        "status": "error",
        "detail": "error description"
}
```
- [] `/admin`: manage users (authenticated)
- [x] `/users?active=true`: list of (active) guests (no auth, lol)
- [] `PUT /door/<id>`: ring (human) or open door (from RPI)  (authenticated?)
  - request
```
{
        "action": ['open', 'ring']
}
```
  - will save door_id+timestamp (after 30 secs delete "rung" will become false)
  - response 201
```
{
        "msg": "opening / ringing door / doorbell not being rung"
}
```
  - response 401
```
{
        "msg": "auth failed"
}
```
- [] `POST /door`: CRUD for doors
  - response 201
```
{
    "id": "123",
    "creation_ts": "1533849560"
}
```
- [] `DELETE /door/<id>`: delete a door
  - response 204

### Check the SQLite file!

`$ sqlite3 app.db`

`sqlite> .tables`

`sqlite> select * from users;`

### Caveats

`rocket` requires the rust `nightly` compiler (which is in flux by definition)
and that gets a bit on the nerves to the `diesel` crate, so you may want to pin
a working combo of a nightly version + `diesel` crate and carefully evaluate
upgrades.

Ensure using a nightly version that goes along with Rocket: check the [Travis CI
config file](https://github.com/SergioBenitez/Rocket/blob/master/.travis.yml).

For support on Rocket ensure to check the log of the [IRC #rocket channel](https://mozilla.logbot.info/rocket).

Q: *There's a warning when I derive Queryable (or other), but my code compiles*
```
5 | derive ( Debug , Clone , Copy , QueryId , Default ) ] pub struct $ column_name
  |                                 ^^^^^^^ names from parent modules are not accessible without an explicit import
```
A: Yeah, we know (see [this
issue](https://github.com/rust-lang/rust/issues/50504#issuecomment-409609119)),
nothing you can do atm, just ignore it or mute with:
```
RUSTFLAGS="-Aproc-macro-derive-resolution-fallback"
```
or better:
``` rust
#[allow(proc_macro_derive_resolution_fallback)]
```

Q: I want to test Rust 2018 preview, break all the things!

A: Sure, why not. Apply [these changes](https://www.ncameron.org/blog/how-to-help-test-the-2018-edition/) and run `cargo run`. If it won't compile, try using [cargo fix](https://rust-lang-nursery.github.io/edition-guide/editions/transitioning.html) to (hopefully) automagically fix your code.
