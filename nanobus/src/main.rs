use tokio;

use nanobus::Server;

#[tokio::main]
async fn main() {
    let (bus, _) = Server::run("/tmp/nanobus.sock").await.unwrap();
    bus.await;
}
