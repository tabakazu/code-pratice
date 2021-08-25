# WebApp Demo

```bash
# setup app
$ cargo new webapp-demo
$ cd webapp-demo/
$ rustup install stable

# setup diesel
$ cargo install diesel_cli --no-default-features --features mysql
$ set -x DATABASE_URL mysql://root:@localhost:3306/rust-webapp-demo
$ diesel setup

# create migration file
$ diesel migration generate create_items

# start webserver
$ cargo run

# start webserver with hot reloader
$ cargo install cargo-watch
$ cargo watch -x run
```

```
$ curl -X GET http://localhost:8080/items/2 | jq
{
  "id": 2,
  "name": "item"
}
```
