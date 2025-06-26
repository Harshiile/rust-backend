use axum::{response::Html, Router,routing::get};


#[tokio::main]
async fn main() {
    let route_method=Router::new().route(
        "/hello",
        get(|| async{
            Html("<h2><u>Hello 19</u></h2>")
        })
    );

    let addr="127.0.0.1:3000";
    
    let listener=tokio::net::TcpListener::bind(addr).await.unwrap();
    
    println!("Server Running On {}",addr);
    axum::serve(listener,route_method).await.unwrap();
}
