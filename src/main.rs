use std::net::SocketAddr;

mod app;
mod endpoint;
mod ioc;
mod observability;

use observability::*;

#[tokio::main]
async fn main() {
    // initialize tracing
    init_tracing();

    // create dependency container
    let container = ioc::create_production_container();

    // build our application with a route
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let app = app::create_app_with_spec(container);
    let app = make_observable(app);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
