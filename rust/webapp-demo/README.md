# WebApp Demo

```bash
$ cargo new webapp-demo
$ cd webapp-demo/
$ rustup install stable
$ cargo run
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
