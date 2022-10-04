# Parviocula

A simple [ASGI](https://asgi.readthedocs.io/en/latest/introduction.html) server aimed at helping the transition from python ASGI applications to an [Axum](https://github.com/tokio-rs/axum) application (or maybe in the future, any [Tower](https://github.com/tower-rs/tower) based web framework).

The goal is to allow writing an Axum based application in rust, with the option of falling back onto the python ASGI application to handle requests not yet implemented in rust, allowing a slow transition from an ASGI based python service to an Axum based rust service without having to do it all in one go.

This takes on the role of the ASGI server you would currently use, using Hyper to handle all the http protocol stuff and converting the requests into ASGI messages using [pyo3](https://github.com/PyO3/pyo3/) and passing them onto the python ASGI application.

This requires writing a small pyo3 based wrapper for your rust server that allows launching it from python. See the [asgi_only](./examples/asgi_only) example for a minimal starting example. The [`README.md`](./examples/asgi_only/README.md) in the examples details the setup for the project as well.

## Limitations

While in most cases you can simply use the `fallback` on the Axum router to forward things not implemented in rust onto the python code, if you have some methods on the same path implemented in both rust and python (e.g. a GET handled by rust, and the POST still handled by python) you need to specificly tell the router to forward the python methods onto the ASGI router. See the [mixed_routes](./examples/mixed_routes) example.

Forwarding routes from nested Axum routers onto the ASGI application will lose the path information from the parent router, and as a result the ASGI app will only have the path information from the current router passed to it. For now it's recommended to only use a single flat router, and wait until you've replaced all the ASGI parts to split the router up if you wish.

## FAQ

### Is it Blazingly Fast?

🤷. Testing on my machine with [oha](https://github.com/hatoo/oha), comparing against [uvicorn](https://github.com/encode/uvicorn), using the [`asgi_example`](./examples/asgi_only), building with `maturin develop --release`, running with `oha http://localhost:3000/echo -z 10s -c 1 -H "user-agent: oha" -d "hello"`, it currently only gets ~80-90% of the requests per second of uvicorn with a single oha worker. But with 50 oha workers (`oha ... -c 50`) it gets ~200% the requests per second of uvicorn. When replacing `uvicorn` in a large suite of integration tests I have that take upwards of 10 minutes to complete, I see no significant difference in the completion speed of the test suite.

### Which ASGI application frameworks does it support?

I've used it with [Starlette](https://github.com/encode/starlette) directly and [FastAPI](https://github.com/tiangolo/fastapi/). I suspect others should work as well, but I've not tested any.

### Can I replace my python ASGI server with this?

Maybe? But unless you're using it to transition your code base to rust it's not worth it. It's probably slower (at least if you use uvicorn like I do) and you'll likely lose a lot of the niceties you get from your current mature ASGI server.

### The `ServerContext` API is terrible! Can you change it?

Feel free to open an Issue with suggestions or a Pull Request with changes and I'll consider it.

### You're doing X wrong, it should be done like this ...!

Again, feel free to open an Issue with suggestions or a Pull Request with changes and I'll look at changing it.

## TODOs

 - Make it faster?
 - Some tests for the ASGI server to make sure it conforms to the specification.
 - Implement as a Tower Service, rather than an Axum Handler
 - Figure out if the `ServerContext::start` function can `await` until the server actually starts listening for requests
 - Starting the server from rust. i.e. being able to more easily replace a python ASGI server as the entry point for starting the application. (this has proven to be tricky due to linking to python directly, and I had troubles starting the asyncio event loop from rust. In the end it's much easier to launch from python and I don't think there's really any benefit to doing it from rust anyway)
 - pyo3_asyncio sometimes generates `InvalidStateError`s due to using `call_soon_threadsafe` to `set_result` on it's futures, and in some cases (that I haven't been able to make a minimal example for yet) the futures are beging cancelled after the `call_soon_threadsafe` call but before the actual `set_result` call it made. It doesn't effect anything (as the futures were cancelled), but is annoying to see the errors in the logs.
 - python typing helpers
 - More tracing support?
 - Websockets?
 - Figure out the OpenAPI story
