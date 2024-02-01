mod in_memory;
use axum::{Json, Router, routing};
use crate::in_memory::load_state;
use serde::Serialize;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello",routing::get(handler))
        .merge(in_memory::rest_router()).with_state(load_state());


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, app.into_make_service()).await.unwrap_or_else(|_| panic!("Server cannot launch."));
}

#[derive(Serialize)]
struct Message{
    message:String
}

async fn handler()-> Json<Message>{
    Json(Message{
        message:String::from("Hello avaya")
    })
}
