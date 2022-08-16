use axum::{handler::Handler, Router};
use parviocula::{AsgiHandler, ServerContext};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyUnicode;
use pyo3::PyResult;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[pyfunction]
fn create_server(
    app: PyObject,
    host: Option<&PyUnicode>,
    port: Option<u16>,
) -> PyResult<Py<ServerContext>> {
    let host = match host {
        Some(host) => IpAddr::V4(
            host.to_string()
                .parse()
                .map_err(|_| PyErr::new::<PyValueError, _>("Invalid host"))?,
        ),
        None => IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
    };
    let port = port.unwrap_or(3000);

    let ctx = parviocula::create_server_context(
        app,
        Box::new(move |asgi: AsgiHandler, rx| async move {
            let app = Router::new().fallback(asgi.into_service());

            let addr = SocketAddr::new(host, port);
            let res = axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .with_graceful_shutdown(async move {
                    if let Err(e) = rx.await {
                        eprintln!("{e}");
                    }
                })
                .await;
            if let Err(err) = res {
                eprintln!("{err}");
            }
        }),
    );
    Ok(ctx)
}

#[pymodule]
fn asgi_only(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_server, m)?)?;
    Ok(())
}
