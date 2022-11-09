use std::sync::Arc;

use async_trait::async_trait;

use super::event_bus::EventBus;

#[async_trait]
pub trait EventRepository<IE, OE, IS, OS> {
    async fn store_events(&self, events: Vec<IE>) -> Result<(), anyhow::Error>;
    async fn retrieve_events(
        &self,
        aggregate_id: String,
        after: Option<String>,
    ) -> Result<Vec<OE>, anyhow::Error>;
    async fn store_snapshot(&self, snapshot: IS) -> Result<(), anyhow::Error>;
    async fn retrieve_latest_snapshot(
        &self,
        aggregate_id: String,
    ) -> Result<Option<OS>, anyhow::Error>;
    // Outbox
    // Used by outbox pattern to retrieve events for sending
    async fn retrieve_outbox_events(&self) -> Result<Vec<OE>, anyhow::Error>;
    // Used by outbox pattern to remove events after sending
    async fn send_and_delete_outbox_event(
        &self,
        event: IE,
        bus: &Arc<dyn EventBus<IE, OE> + Send + Sync>,
    ) -> Result<(), anyhow::Error>;
}
