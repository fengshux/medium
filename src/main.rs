use axum::{
    handler::{get, post},
    http::{StatusCode, Request, header::{HeaderMap, HeaderName, HeaderValue}},
    response::IntoResponse,
    Json, Router,
    service,
    routing::BoxRoute
};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, net::SocketAddr};
use tower_http::{services::ServeDir, trace::TraceLayer};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()    
        // .route("/", get(root))    
        // .route("/files", get(files))
        .route(
            "/static",
            get(read_file)
        );

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn read_file() -> (HeaderMap, Vec<u8>) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("image/jpeg"),
    );
    let filename = "pictures/Horseshoe Bend, U.S..jpg".to_string();
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");    
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    
    (headers, buffer)
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}


async fn files() -> Json<Vec<String>> {
    let paths = fs::read_dir("pictures").unwrap();

    let list = paths.map(|path| { path.unwrap().file_name().into_string().map_or("".to_string(), |x| x ) });
    
    return Json(list.collect());
}
