use std::sync::Arc;

use async_trait::async_trait;
use futures::Stream;

#[async_trait]
pub trait EventBus<IE, OE> {
    async fn send_event(&self, event: IE) -> Result<(), anyhow::Error>;
    async fn receive_events(&self) -> Box<dyn Stream<Item = OE>>;
}
