use std::time::Duration;

use axact::{pages, realtime, AppState, Snapshot};
use axum::{Router, Server};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<Snapshot>(1);
    let app_state = AppState { tx: tx.clone() };

    let router = Router::new()
        .nest("/", pages::router())
        .nest("/realtime", realtime::router(app_state));

    let duration = Duration::from_millis(250);
    tokio::task::spawn_blocking(move || {
        Snapshot::usage_transmitter(tx, duration)
    });

    let addr = &"0.0.0.0:7032".parse().unwrap();
    let router_service = router.into_make_service();
    let server = Server::bind(addr).serve(router_service);

    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap()
}
