use crate::types::Collector;
use crate::CollectorStream;
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::connect_async;
use tungstenite::client::IntoClientRequest;
use tungstenite::Message;

pub struct SolanaCollector {
    url: String,
}

impl SolanaCollector {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[async_trait]
impl Collector<Message> for SolanaCollector {
    fn name(&self) -> &str {
        "SolanaCollector"
    }

    async fn get_event_stream(&self) -> eyre::Result<CollectorStream<'_, Message>> {
        let request = self
            .url
            .clone()
            .into_client_request()
            .expect("Failed to create request");

        let (ws_stream, _) = connect_async(request).await.expect("Failed to connect");
        let (mut write, mut read) = ws_stream.split();

        let subscribe_request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "logsSubscribe",
            "params": [
                {
                    "mentions": ["E645TckHQnDcavVv92Etc6xSWQaq8zzPtPRGBheviRAk"]
                },
                {
                    "commitment": "finalized"
                }
            ]
        });

        // send subscription request
        write
            .send(Message::Text(subscribe_request.to_string()))
            .await
            .expect("Failed to send subscribe request");

        let stream = async_stream::stream! {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(message) => yield message,
                    Err(e) => println!("Error receiving message: {}", e),
                }
            }
        };

        Ok(Box::pin(stream))
    }
}
