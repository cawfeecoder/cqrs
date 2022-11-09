use std::collections::HashMap;
use std::fmt::Debug;

use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::context::common::domain::entity::{
    aggregate::Aggregate,
    event::{AggregateSnapshot, EventEnvelope},
};

#[derive(FromRow, Debug)]
pub struct SQLEventEnvelope<A>
where
    A: Default,
{
    /// The id of the aggregate instance.
    #[sqlx(default)]
    pub aggregate_id: String,
    /// The type of aggregate instance
    #[sqlx(default)]
    pub aggregate_type: String,
    /// The sequence id for an aggregate instance.
    #[sqlx(default)]
    pub sequence: String,
    /// The event payload with all business information.
    #[sqlx(default)]
    pub payload: sqlx::types::Json<A>,
    /// Additional metadata for use in auditing, logging or debugging purposes.
    #[sqlx(default)]
    pub metadata: sqlx::types::Json<HashMap<String, String>>,
    /// Timestamp of when this event was produced
    #[sqlx(default)]
    pub timestamp: DateTime<Utc>,
}

impl<A: Default + Debug + Into<B::Event>, B: Aggregate> Into<EventEnvelope<B>>
    for SQLEventEnvelope<A>
{
    fn into(self) -> EventEnvelope<B> {
        return EventEnvelope {
            aggregate_id: self.aggregate_id,
            aggregate_type: self.aggregate_type,
            sequence: self.sequence,
            payload: self.payload.0.into(),
            metadata: self.metadata.0,
            timestamp: self.timestamp,
        };
    }
}

#[derive(FromRow, Debug)]
pub struct SQLAggregateSnapshot<Q>
where
    Q: Default,
{
    /// The id of the aggregate instance.
    #[sqlx(default)]
    pub aggregate_id: String,
    /// The type of aggregate instance
    #[sqlx(default)]
    pub aggregate_type: String,
    /// The event payload with all business information.
    #[sqlx(default)]
    pub payload: sqlx::types::Json<Q>,
    /// The last committed event sequence ULID for this aggregate instance.
    #[sqlx(default)]
    pub last_sequence: String,
    /// Additional metadata for use in auditing, logging or debugging purposes.
    #[sqlx(default)]
    pub snapshot_id: String,
    /// Timestamp of when this event was produced
    #[sqlx(default)]
    pub timestamp: DateTime<Utc>,
}

impl<Q: Default + Debug + Into<S>, S: Aggregate> Into<AggregateSnapshot<S>>
    for SQLAggregateSnapshot<Q>
{
    fn into(self) -> AggregateSnapshot<S> {
        return AggregateSnapshot {
            aggregate_id: self.aggregate_id,
            aggregate_type: self.aggregate_type,
            payload: self.payload.0.into(),
            last_sequence: self.last_sequence,
            snapshot_id: self.snapshot_id,
            timestamp: self.timestamp,
        };
    }
}
