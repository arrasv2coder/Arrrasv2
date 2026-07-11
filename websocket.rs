use axum::{
    routing::get,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    response::Html,
    response::Response,
    extract::{State,Query,DefaultBodyLimit},
    Router,
};
use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use axum::extract::ws::WebSocketUpgrade;
use tokio::{fs,sync::oneshot,sync::Mutex};
use std::sync::Arc;
use std::thread;
use serde::Deserialize;
#[derive(Deserialize)]
struct SearchParams {
    query: String,
}
#[tokio::main]
async fn main() {
    let app = Router::new().route("/",axum::routing::get(connect)).route("/ws", get(websocket_handler));;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
async fn connect()-> Html<&'static str> {
    Html(r#"<div id='error-container'>
    Loading websocket, hack this at your own risk...
    </div>
    <script>
    const socket = new WebSocket('ws://localhost:3000/ws?query=0000');
    socket.binaryType = 'arraybuffer';
    socket.onerror = async (error) => {
    document.getElementById('error-container').innerHTML ='<h1>Nice try hacker</h1><iframe src="https://www.youtube.com/embed/3avG4gmlA-o?autoplay=1&mute=1" width="600" height="400" title="Well you deserve it"></iframe>';
    }</script>"#)
}
async fn websocket_handler(ws:WebSocketUpgrade,Query(params): Query<SearchParams>) -> impl IntoResponse {
    if (params.query=="0000"){
        ws.on_upgrade(move |socket| handle_socket(socket, params.query))
    }
    else {
        (StatusCode::FORBIDDEN).into_response()
    }
}
async fn handle_socket(mut socket: WebSocket, user_id:String) {
    if let Err(e) = socket
        .send(Message::Text("Hello from the server!".to_string().into()))
        .await
    {
        eprintln!("Error sending message: {}", e);
        return;
    }
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Binary(msg) => {
                println!("Received message");
            }
            Message::Close(_) => {
                println!("Closing WebSocket connection.");
                break;
            }
            _ => {}
        }
    }
}

