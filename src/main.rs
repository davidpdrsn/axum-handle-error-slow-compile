use axum::{http::StatusCode, prelude::*};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Duration;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let app = route("/", get(handler))
        .nest(
            "/api",
            route("/pay/get_pay_params", post(handler))
                .nest(
                    "/user",
                    route("/create", post(handler))
                        .route("/login", post(handler))
                        .route("/info", get(handler))
                        .route("/update_password", get(handler)),
                )
                .nest(
                    "/product",
                    route("/list", get(handler)).route("/detail", get(handler)),
                ),
        )
        .layer(
            ServiceBuilder::new()
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let app = app.handle_error(|error: BoxError| {
        if error.is::<tower::timeout::error::Elapsed>() {
            Ok::<_, Infallible>((
                StatusCode::REQUEST_TIMEOUT,
                "request took too long".to_string(),
            ))
        } else {
            Ok::<_, Infallible>((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            ))
        }
    });

    let addr: SocketAddr = "127.0.0.1".parse::<SocketAddr>().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "index"
}
