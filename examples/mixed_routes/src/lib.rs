use std::net::SocketAddr;

use axum::{extract::Query, response::IntoResponse, routing::get, Router};
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
        .fallback(asgi);
    let addr = SocketAddr::new([127, 0, 0, 1].into(), port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    if let Err(err) = axum::serve(listener, app)
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
#[pyo3(signature = (app, port=None))]
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
fn mixed_routes(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_server, m.clone())?)?;
    Ok(())
}
