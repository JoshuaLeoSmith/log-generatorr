mod generator;
mod messages;
mod rotation;
mod server;

use std::sync::Arc;

use generator::GeneratorState;

#[tokio::main]
async fn main() {
    let state = Arc::new(GeneratorState::new());
    let app = server::create_router(state);

    let addr = "0.0.0.0:3000";
    println!("╔══════════════════════════════════════════╗");
    println!("║       Log Generator is running!          ║");
    println!("║  Open http://localhost:3000 in browser    ║");
    println!("╚══════════════════════════════════════════╝");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
