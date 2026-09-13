use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let router01 = Router::new().route("/vehicle", get(get_vehicle));

    let address = "0.0.0.0:6570";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, router01).await.unwrap();
}

async fn get_vehicle() {

}
