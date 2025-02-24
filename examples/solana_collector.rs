use mangoflow::collector::SolanaCollector;
use mangoflow::{
    map_collector, map_executor, submit_action, ActionSubmitter, Executor, MangoflowEngine,
    Strategy,
};
use mimalloc::MiMalloc;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use tungstenite::Message;

#[derive(Debug, Clone)]
enum Action {
    EchoMessage(String),
}

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Debug, Clone)]
enum Event {
    Message(Message),
}

pub struct EchoStrategy;

#[async_trait::async_trait]
impl Strategy<Event, Action> for EchoStrategy {
    async fn process_event(&mut self, event: Event, submitter: Arc<dyn ActionSubmitter<Action>>) {
        match event {
            Event::Message(message) => {
                submit_action!(submitter, Action::EchoMessage, message.into_text().unwrap())
            }
        }
    }
}

#[derive(Default)]
pub struct EchoExecutor<T>(PhantomData<T>);

#[async_trait::async_trait]
impl<T: Debug + Send + Sync> Executor<T> for EchoExecutor<T> {
    async fn execute(&self, action: T) -> eyre::Result<()> {
        println!("action: {:?}", action);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut engine = MangoflowEngine::new();
    let solana_collector = SolanaCollector::new("wss://solana.drpc.org".to_string());

    // add collectors
    engine.add_collector(map_collector!(solana_collector, Event::Message));

    // add strategy
    engine.add_strategy(Box::new(EchoStrategy));

    // add executors
    engine.add_executor(map_executor!(EchoExecutor::default(), Action::EchoMessage));

    engine.run_and_join().await.unwrap();
}
