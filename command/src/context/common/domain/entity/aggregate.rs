use std::error::Error;

use super::event::{AggregateSnapshot, DomainEvent};
use async_trait::async_trait;

#[async_trait]
pub trait Aggregate: Default + Sync + Send {
    type Command;
    type Event: DomainEvent;
    type Error: Error;
    type Services: Send + Sync;

    fn aggregate_type() -> String;

    fn aggregate_id(&self) -> Option<String>;

    async fn handle(
        &self,
        command: Self::Command,
        service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error>;

    fn apply(&mut self, event: Self::Event);

    fn snapshot(&mut self) -> Option<AggregateSnapshot<Self>>;
}
