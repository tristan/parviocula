## Mixed Routes

Demos handling the same uri path with both python and rust.

## Project setup

Requires [Maturin](https://github.com/PyO3/maturin)

```
pip install maturin
```

```
maturin new -b pyo3 mixed_routes
cd mixed_routes
cargo add --path ../../ -F extension-module
cargo add pyo3 -F extension-module
cargo add tokio -F full
cargo add axum
cargo add serde -F derive

< write code >
```

## Running using venv

```
python -m venv .venv
source .venv/bin/activate
pip install maturin
pip install starlette
maturin develop
python demo.py

...

$ curl http://localhost:3000/post_or_get?name=bob
Hello bob, from rust!
$ curl -H 'Content-Type: application/json' -d '{"name": "bob"}' http://localhost:3000/post_or_get
Hello bob, from python!
```
