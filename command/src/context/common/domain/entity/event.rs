use chrono::{DateTime, Utc};
use std::{collections::HashMap, fmt::Debug};

use super::aggregate::Aggregate;

pub trait DomainEvent: Clone + PartialEq + Debug + Sync + Send {
    fn event_type(&self) -> String;

    fn event_version(&self) -> String;

    fn event_id(&self) -> String;
}

#[derive(Debug)]
pub struct EventEnvelope<A>
where
    A: Aggregate,
{
    /// The id of the aggregate instance.
    pub aggregate_id: String,
    /// The type of aggregate instance
    pub aggregate_type: String,
    /// The sequence id for an aggregate instance.
    pub sequence: String,
    /// The event payload with all business information.
    pub payload: A::Event,
    /// Additional metadata for use in auditing, logging or debugging purposes.
    pub metadata: HashMap<String, String>,
    /// Timestamp of when this event was produced
    pub timestamp: DateTime<Utc>,
}

impl<A: Aggregate> Clone for EventEnvelope<A> {
    fn clone(&self) -> Self {
        EventEnvelope {
            aggregate_id: self.aggregate_id.clone(),
            aggregate_type: self.aggregate_type.clone(),
            sequence: self.sequence.clone(),
            payload: self.payload.clone(),
            metadata: self.metadata.clone(),
            timestamp: self.timestamp.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AggregateSnapshot<S> {
    /// The id of the aggregate instance that this is a snapshot for
    pub aggregate_id: String,
    /// The type of aggregate instance
    pub aggregate_type: String,
    /// The current state of the aggregate instance (e.g. the snapshot data)
    pub payload: S,
    /// The last committed event sequence ULID for this aggregate instance.
    pub last_sequence: String,
    /// The id of this snapshot
    pub snapshot_id: String,
    /// Timestamp of when this event was produced
    pub timestamp: DateTime<Utc>,
}
