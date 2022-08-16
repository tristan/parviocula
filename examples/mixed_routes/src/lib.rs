use std::net::SocketAddr;

use axum::{extract::Query, handler::Handler, response::IntoResponse, routing::get, Router};
use parviocula::{AsgiHandler, ServerContext};
use pyo3::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct RootQuery {
    name: String,
}

async fn get_root(Query(RootQuery { name }): Query<RootQuery>) -> impl IntoResponse {
    format!("Hello {name}, from rust!")
}

async fn start(port: u16, shutdown_signal: tokio::sync::oneshot::Receiver<()>, asgi: AsgiHandler) {
    let app = Router::new()
        .route("/post_or_get", get(get_root).post(asgi.clone()))
        .fallback(asgi.into_service());
    let addr = SocketAddr::new([127, 0, 0, 1].into(), port);
    if let Err(err) = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async move {
            if let Err(err) = shutdown_signal.await {
                eprintln!("failed to send shutdown signal: {err}");
            }
        })
        .await
    {
        eprintln!("error running server: {err}");
    };
}

#[pyfunction]
fn create_server(app: PyObject, port: Option<u16>) -> PyResult<Py<ServerContext>> {
    let port = port.unwrap_or(3000);
    let ctx = parviocula::create_server_context(
        app,
        Box::new(move |asgi, rx| async move {
            start(port, rx, asgi).await;
        }),
    );
    Ok(ctx)
}

#[pymodule]
fn mixed_routes(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_server, m)?)?;
    Ok(())
}
