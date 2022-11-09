use crate::context::common::domain::entity::event::EventEnvelope;
use crate::context::prescription::domain::entity::aggregate::PrescriptionAggregate;
use async_trait::async_trait;

#[async_trait]
pub trait GetEvents<O>
where
    O: Into<EventEnvelope<PrescriptionAggregate>>,
{
    async fn get_events(&self) -> Result<Vec<O>, anyhow::Error>;
}
