use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;

use crate::context::common::domain::entity::aggregate::{self, Aggregate};
use crate::context::common::domain::entity::event::DomainEvent;
use crate::context::common::domain::entity::event::{AggregateSnapshot, EventEnvelope};
use crate::context::common::domain::ports::outbound::event_repository::EventRepository;
use crate::context::prescription::application::ports::inbound::create_prescription::CreatePrescriptionUseCase;
use crate::context::prescription::application::ports::inbound::update_prescription::UpdatePrescriptionUseCase;
use crate::context::prescription::application::ports::outbound::prescription::PrescriptionServices;
use crate::context::prescription::domain::entity::aggregate::PrescriptionAggregate;
use crate::context::prescription::domain::entity::command::{
    CreatePrescriptionCommand, PrescriptionCommand, UpdatePrescriptionCommand,
};
use crate::context::prescription::domain::entity::error::PrescriptionError;

pub trait ServiceTrait<O: From<PrescriptionAggregate>>:
    CreatePrescriptionUseCase<O> + UpdatePrescriptionUseCase<O>
{
}

pub struct PrescriptionService {
    services: Box<dyn Sync + Send + PrescriptionServices>,
    repository: Arc<
        dyn EventRepository<
                EventEnvelope<PrescriptionAggregate>,
                EventEnvelope<PrescriptionAggregate>,
                AggregateSnapshot<PrescriptionAggregate>,
                AggregateSnapshot<PrescriptionAggregate>,
            > + Sync
            + Send,
    >,
}

impl PrescriptionService {
    pub fn new(
        services: Box<dyn Sync + Send + PrescriptionServices>,
        repository: Arc<
            dyn EventRepository<
                    EventEnvelope<PrescriptionAggregate>,
                    EventEnvelope<PrescriptionAggregate>,
                    AggregateSnapshot<PrescriptionAggregate>,
                    AggregateSnapshot<PrescriptionAggregate>,
                > + Sync
                + Send,
        >,
    ) -> Self {
        return Self {
            services,
            repository,
        };
    }
}

#[async_trait]
impl<O> CreatePrescriptionUseCase<O> for PrescriptionService
where
    O: From<PrescriptionAggregate>,
{
    async fn create_prescription(
        &self,
        prescription: CreatePrescriptionCommand,
        _fields: Vec<&str>,
    ) -> Result<O, anyhow::Error> {
        let mut aggregate = PrescriptionAggregate::default();
        let command: PrescriptionCommand = prescription.into();
        let events = aggregate.handle(command, &self.services).await;
        match events {
            Ok(v) => {
                for event in &v {
                    aggregate.apply(event.clone());
                }
                if aggregate.aggregate_id().is_none() {
                    return Err(PrescriptionError::UnknownError.into());
                }
                let wrapped_events: Vec<EventEnvelope<PrescriptionAggregate>> = v
                    .iter()
                    .map(|x| EventEnvelope::<PrescriptionAggregate> {
                        aggregate_id: aggregate.aggregate_id().unwrap(),
                        aggregate_type: "prescription".into(),
                        sequence: x.event_id(),
                        payload: x.clone(),
                        metadata: HashMap::new(),
                        timestamp: Utc::now(),
                    })
                    .collect();
                let result = self.repository.store_events(wrapped_events).await;
                match result {
                    Err(_) => return Err(PrescriptionError::UnknownError.into()),
                    _ => {}
                }
                match aggregate.snapshot() {
                    Some(x) => match self.repository.store_snapshot(x).await {
                        Err(_e) => println!("Failed to persist snapshot"),
                        _ => {}
                    },
                    None => {}
                }
                return Ok(aggregate.into());
            }
            Err(e) => return Err(e.into()),
        }
    }
}

#[async_trait]
impl<O> UpdatePrescriptionUseCase<O> for PrescriptionService
where
    O: From<PrescriptionAggregate>,
{
    async fn update_prescription(
        &self,
        prescription: UpdatePrescriptionCommand,
        _fields: Vec<&str>,
    ) -> Result<O, anyhow::Error> {
        let mut aggregate = PrescriptionAggregate::default();
        let snapshot = self
            .repository
            .retrieve_latest_snapshot(prescription.id.clone())
            .await?;
        let past_events = match snapshot {
            Some(x) => {
                aggregate = x.payload;
                self.repository
                    .retrieve_events(prescription.id.clone(), Some(x.last_sequence))
                    .await?
            }
            None => {
                self.repository
                    .retrieve_events(prescription.id.clone(), None)
                    .await?
            }
        };
        for event in past_events.iter() {
            aggregate.apply(event.payload.clone());
        }
        let command: PrescriptionCommand = prescription.into();
        let events = aggregate.handle(command, &self.services).await;
        match events {
            Ok(v) => {
                for event in &v {
                    aggregate.apply(event.clone());
                }
                if aggregate.aggregate_id().is_none() {
                    return Err(PrescriptionError::UnknownError.into());
                }
                let wrapped_events: Vec<EventEnvelope<PrescriptionAggregate>> = v
                    .iter()
                    .map(|x| EventEnvelope::<PrescriptionAggregate> {
                        aggregate_id: aggregate.aggregate_id().unwrap(),
                        aggregate_type: "prescription".into(),
                        sequence: x.event_id(),
                        payload: x.clone(),
                        metadata: HashMap::new(),
                        timestamp: Utc::now(),
                    })
                    .collect();
                let result = self.repository.store_events(wrapped_events).await;
                match result {
                    Err(_) => return Err(PrescriptionError::UnknownError.into()),
                    _ => {}
                }
                match aggregate.snapshot() {
                    Some(x) => match self.repository.store_snapshot(x).await {
                        Err(_e) => println!("Failed to persist snapshot"),
                        _ => {}
                    },
                    None => {}
                }
                return Ok(aggregate.into());
            }
            Err(e) => return Err(e.into()),
        }
    }
}

impl<O: From<PrescriptionAggregate>> ServiceTrait<O> for PrescriptionService {}
