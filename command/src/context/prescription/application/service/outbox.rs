use std::sync::Arc;

use crate::context::{
    common::application::ports::outbound::{
        event_bus::EventBus, event_repository::EventRepository,
    },
    common::domain::entity::event::AggregateSnapshot,
    common::domain::entity::event::EventEnvelope,
    prescription::application::ports::inbound::{get_events::GetEvents, send_event::SendEvent},
    prescription::domain::entity::aggregate::PrescriptionAggregate,
};

use async_trait::async_trait;

pub struct PrescriptionOutboxService {
    repository: Arc<
        dyn EventRepository<
                EventEnvelope<PrescriptionAggregate>,
                EventEnvelope<PrescriptionAggregate>,
                AggregateSnapshot<PrescriptionAggregate>,
                AggregateSnapshot<PrescriptionAggregate>,
            > + Sync
            + Send,
    >,
    bus: Arc<
        dyn EventBus<EventEnvelope<PrescriptionAggregate>, EventEnvelope<PrescriptionAggregate>>
            + Sync
            + Send,
    >,
}

impl PrescriptionOutboxService {
    pub fn new(
        repository: Arc<
            dyn EventRepository<
                    EventEnvelope<PrescriptionAggregate>,
                    EventEnvelope<PrescriptionAggregate>,
                    AggregateSnapshot<PrescriptionAggregate>,
                    AggregateSnapshot<PrescriptionAggregate>,
                > + Sync
                + Send,
        >,
        bus: Arc<
            dyn EventBus<EventEnvelope<PrescriptionAggregate>, EventEnvelope<PrescriptionAggregate>>
                + Sync
                + Send,
        >,
    ) -> Self {
        return Self { repository, bus };
    }
}

#[async_trait]
impl GetEvents<EventEnvelope<PrescriptionAggregate>> for PrescriptionOutboxService {
    async fn get_events(&self) -> Result<Vec<EventEnvelope<PrescriptionAggregate>>, anyhow::Error> {
        self.repository.retrieve_outbox_events().await
    }
}

#[async_trait]
impl SendEvent<EventEnvelope<PrescriptionAggregate>> for PrescriptionOutboxService {
    async fn send_event(
        &self,
        event: EventEnvelope<PrescriptionAggregate>,
    ) -> Result<(), anyhow::Error> {
        self.repository
            .send_and_delete_outbox_event(event, &self.bus)
            .await
    }
}
