## Asgi Only

Demos simply running a python asgi app with an axum based asgi server.

## Project setup


Requires [Maturin](https://github.com/PyO3/maturin)

```
pip install maturin
```

```
maturin new -b pyo3 asgi_only
cd asgi_only
cargo add --path ../../ -F extension-module
cargo add pyo3 -F extension-module
cargo add axum

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

$ curl http://localhost:3000/simple
HELLO WORLD!
```
