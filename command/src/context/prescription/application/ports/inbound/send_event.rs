use crate::context::common::domain::entity::event::EventEnvelope;
use crate::context::prescription::domain::entity::aggregate::PrescriptionAggregate;
use async_trait::async_trait;

#[async_trait]
pub trait SendEvent<I>
where
    I: Into<EventEnvelope<PrescriptionAggregate>>,
{
    async fn send_event(&self, event: I) -> Result<(), anyhow::Error>;
}
