use engine::events::EngineEvent;
use tokio::sync::broadcast;

use crate::utils;

pub async fn run(feed_tx: broadcast::Sender<Vec<EngineEvent>>) {
    let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let multicast_addr = "224.0.1.1:1338";

    let mut rx = feed_tx.subscribe();

    while let Ok(events) = rx.recv().await {
        for event in events {
            let _ = socket.send_to(&utils::engine_event_as_bytes(&event).0, multicast_addr).await;
        }
    }
}
