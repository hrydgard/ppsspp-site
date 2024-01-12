use axum::{
    body::Body,
    extract::{Request, State},
    http::{uri::Uri, HeaderValue},
    response::{IntoResponse, Response},
    routing::{on, MethodFilter},
    Router,
};
use hyper::StatusCode;
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type Client = hyper_util::client::legacy::Client<HttpConnector, Body>;

pub async fn spawn_server(port: u16) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tokio::spawn(server(port));
}

async fn server(port: u16) {
    let client: Client =
        hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
            .build(HttpConnector::new());

    let app = Router::new().nest_service("/", ServeDir::new("build"));
    let app = app
        .route(
            "/api/*path",
            on(
                MethodFilter::GET.or(MethodFilter::POST),
                reverse_proxy_handler,
            ),
        )
        .with_state(client);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}

async fn reverse_proxy_handler(
    State(client): State<Client>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let path = req.uri().path();
    let path_query = req.uri().path_and_query().map_or(path, |v| v.as_str());

    // let proxy_url = "http://127.0.0.1:3000";
    // let proxy_url = "http://46.246.45.220:9101";
    let proxy_url = "http://centraldev.ppsspp.org";

    let uri = format!("{}{}", proxy_url, path_query);

    *req.uri_mut() = Uri::try_from(uri).unwrap();
    if let Some(host) = req.headers_mut().get_mut("host") {
        *host = HeaderValue::from_static("centraldev.ppsspp.org:80");
    }

    println!("{:#?}", *req.headers());

    Ok(client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_response())
}
