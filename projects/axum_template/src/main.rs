#![allow(unused)] //For begenning only

use std::net::SocketAddr;

use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
   let routers = Router::new().route(
    "/hello",
    get(|| async {Html("Hello <strong>World!!!</strong>")}),
   );

   let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
   println!("LISTENING on {:?}\n", addr);
   axum::Server::bind(&addr)
    .serve(routers.into_make_service())
    .await.unwrap()
}
