use axum::{ routing::get, Json, Router};
use serde_json::json;

#[tokio::main]
async fn main() {
    let route_method=Router::new().route(
        "/hello",
        get(|| async{
            Json(json!({
                "message":"hjavhdh"
            }))
        })
    );

    let addr="127.0.0.1:3000";
    
    let listener=tokio::net::TcpListener::bind(addr).await.unwrap();
    
    println!("Server Running On {}",addr);
    axum::serve(listener,route_method).await.unwrap();
}
