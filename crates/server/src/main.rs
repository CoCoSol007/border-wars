use axum::extract::ws::WebSocket;
use axum::extract::WebSocketUpgrade;
use axum::routing::get;
use axum::Router;

mod game;
mod login;

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(|ws: WebSocketUpgrade| async {
            ws.on_upgrade(|socket| async {
                if let Err(e) = handle(socket).await {
                    eprintln!("Error: {}", e);
                }
            })
        }),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .expect("failed to bind");
    axum::serve(listener, app).await.expect("failed to serve");
}

async fn handle(mut socket: WebSocket) -> anyhow::Result<()> {
    Ok(())
}
