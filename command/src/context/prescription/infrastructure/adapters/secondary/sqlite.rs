use std::sync::Arc;

use async_trait::async_trait;
use serde_json::json;
use sqlx::Sqlite;

use crate::context::{
    common::{
        application::ports::outbound::{event_bus::EventBus, event_repository::EventRepository},
        domain::entity::event::{AggregateSnapshot, DomainEvent, EventEnvelope},
        infrastructure::{
            adapters::secondary::storage::sqlite::SqliteConnector,
            dtos::storage::sql::{SQLAggregateSnapshot, SQLEventEnvelope},
        },
    },
    prescription::{
        domain::entity::{aggregate::PrescriptionAggregate, error::PrescriptionError},
        infrastructure::dtos::storage::sql::{SQLPrescriptionAggregate, SQLPrescriptionEvent},
    },
};

const EVENT_TABLE_NAME: &str = "events";
const SNAPSHOT_TABLE_NAME: &str = "snapshots";
const OUTBOX_TABLE_NAME: &str = "outbox_events";

#[async_trait]
impl
    EventRepository<
        EventEnvelope<PrescriptionAggregate>,
        EventEnvelope<PrescriptionAggregate>,
        AggregateSnapshot<PrescriptionAggregate>,
        AggregateSnapshot<PrescriptionAggregate>,
    > for SqliteConnector
{
    async fn store_events(
        &self,
        events: Vec<EventEnvelope<PrescriptionAggregate>>,
    ) -> Result<(), anyhow::Error> {
        let fields = vec![
            "aggregate_type",
            "aggregate_id",
            "sequence",
            "event_type",
            "event_version",
            "payload",
            "metadata",
            "timestamp",
        ];
        let placeholders: Vec<String> = (0..fields.len())
            .map(|x| format!("?{}", (x + 1).to_string()))
            .collect();
        let placeholder_str = placeholders.join(", ");
        let query = format!(
            "INSERT INTO {} ({}) VALUES ( {} )",
            EVENT_TABLE_NAME,
            fields.join(", "),
            placeholder_str
        );
        let outbox_query = format!(
            "INSERT INTO {} ({}) VALUES ( {} )",
            OUTBOX_TABLE_NAME,
            fields.join(", "),
            placeholder_str
        );
        let mut results = vec![];
        for x in events {
            let mut tx = self.pool.begin().await?;
            let plan = sqlx::query::<Sqlite>(&query);
            let outbox_plan = sqlx::query::<Sqlite>(&outbox_query);
            let enum_sql: SQLPrescriptionEvent = x.payload.clone().into();
            let insert = plan
                .bind(&x.aggregate_type)
                .bind(&x.aggregate_id)
                .bind(&x.sequence)
                .bind(&x.payload.event_type())
                .bind(&x.payload.event_version())
                .bind(json!(enum_sql).to_string())
                .bind(json!(x.metadata).to_string())
                .bind(&x.timestamp.to_rfc3339())
                .execute(&mut tx)
                .await;
            let outbox_insert = outbox_plan
                .bind(&x.aggregate_type)
                .bind(&x.aggregate_id)
                .bind(&x.sequence)
                .bind(&x.payload.event_type())
                .bind(&x.payload.event_version())
                .bind(json!(enum_sql).to_string())
                .bind(json!(x.metadata).to_string())
                .bind(&x.timestamp.to_rfc3339())
                .execute(&mut tx)
                .await;
            if outbox_insert.is_err() {
                results.push(outbox_insert)
            }
            tx.commit().await?;
            results.push(insert);
        }
        let mut err: Vec<anyhow::Error> = vec![];
        for result in results {
            match result {
                Err(e) => err.push(e.into()),
                _ => {}
            }
        }
        if err.len() > 0 {
            return Err(PrescriptionError::UnknownError.into());
        }
        return Ok(());
    }

    async fn retrieve_events(
        &self,
        aggregate_id: String,
        after: Option<String>,
    ) -> Result<Vec<EventEnvelope<PrescriptionAggregate>>, anyhow::Error> {
        let fields = vec![
            "aggregate_type",
            "aggregate_id",
            "sequence",
            "event_type",
            "event_version",
            "payload",
            "metadata",
            "timestamp",
        ];
        let query = match after {
            None => format!(
                "SELECT {} FROM {} WHERE aggregate_id = ?1",
                fields.join(", "),
                EVENT_TABLE_NAME
            ),
            Some(_) => format!(
                "SELECT {} FROM {} WHERE aggregate_id = ?1 AND sequence > ?2 ORDER BY sequence ASC",
                fields.join(", "),
                EVENT_TABLE_NAME
            ),
        };
        let mut plan = sqlx::query_as::<Sqlite, SQLEventEnvelope<SQLPrescriptionEvent>>(&query);
        plan = match after {
            None => plan.bind(aggregate_id),
            Some(x) => plan.bind(aggregate_id).bind(x),
        };
        let results = plan.fetch_all(&self.pool).await;
        match results {
            Err(e) => return Err(e.into()),
            _ => {}
        };
        let mut resp: Vec<EventEnvelope<PrescriptionAggregate>> = vec![];
        for env in results.unwrap() {
            let x = env.into();
            resp.push(x)
        }
        return Ok(resp);
    }

    async fn store_snapshot(
        &self,
        snapshot: AggregateSnapshot<PrescriptionAggregate>,
    ) -> Result<(), anyhow::Error> {
        let fields = vec![
            "aggregate_type",
            "aggregate_id",
            "payload",
            "last_sequence",
            "snapshot_id",
            "timestamp",
        ];
        let placeholders: Vec<String> = (0..fields.len())
            .map(|x| format!("?{}", (x + 1).to_string()))
            .collect();
        let placeholder_str = placeholders.join(", ");
        let query = format!(
            "INSERT INTO {} ({}) VALUES ( {} )",
            SNAPSHOT_TABLE_NAME,
            fields.join(", "),
            placeholder_str
        );
        let plan = sqlx::query::<Sqlite>(&query);
        let enum_sql: SQLPrescriptionAggregate = snapshot.payload.clone().into();
        let insert = plan
            .bind(snapshot.aggregate_type)
            .bind(snapshot.aggregate_id)
            .bind(json!(enum_sql).to_string())
            .bind(snapshot.last_sequence)
            .bind(snapshot.snapshot_id)
            .bind(snapshot.timestamp)
            .fetch_optional(&self.pool)
            .await;
        match insert {
            Err(_e) => return Err(PrescriptionError::UnknownError.into()),
            _ => return Ok(()),
        }
    }

    async fn retrieve_latest_snapshot(
        &self,
        aggregate_id: String,
    ) -> Result<Option<AggregateSnapshot<PrescriptionAggregate>>, anyhow::Error> {
        let fields = vec![
            "aggregate_type",
            "aggregate_id",
            "payload",
            "last_sequence",
            "snapshot_id",
            "timestamp",
        ];
        let query = format!(
            "SELECT {} FROM {} WHERE aggregate_id = ?1 ORDER BY snapshot_id DESC LIMIT 1",
            fields.join(", "),
            SNAPSHOT_TABLE_NAME
        );
        let plan = sqlx::query_as::<Sqlite, SQLAggregateSnapshot<SQLPrescriptionAggregate>>(&query)
            .bind(aggregate_id);
        let result = plan.fetch_optional(&self.pool).await;
        match result {
            Err(e) => return Err(e.into()),
            _ => {}
        };
        match result.unwrap() {
            None => Ok(None),
            Some(x) => Ok(Some(x.into())),
        }
    }

    async fn send_and_delete_outbox_event(
        &self,
        event: EventEnvelope<PrescriptionAggregate>,
        bus: &Arc<
            dyn EventBus<EventEnvelope<PrescriptionAggregate>, EventEnvelope<PrescriptionAggregate>>
                + Sync
                + Send,
        >,
    ) -> Result<(), anyhow::Error> {
        let query = format!("DELETE FROM {} WHERE sequence = ?1", OUTBOX_TABLE_NAME);
        let mut plan = sqlx::query::<Sqlite>(&query);
        plan = plan.bind(event.sequence.clone());
        let mut tx = self.pool.begin().await?;
        bus.send_event(event)?;
        let result = plan.execute(&mut tx).await;
        match result {
            Err(e) => {
                tx.rollback().await?;
                return Err(e.into());
            }
            _ => {
                tx.commit().await?;
                return Ok(());
            }
        }
    }

    async fn retrieve_outbox_events(
        &self,
    ) -> Result<Vec<EventEnvelope<PrescriptionAggregate>>, anyhow::Error> {
        let fields = vec![
            "aggregate_type",
            "aggregate_id",
            "sequence",
            "event_type",
            "event_version",
            "payload",
            "metadata",
            "timestamp",
        ];
        let query = format!("SELECT {} FROM {}", fields.join(", "), OUTBOX_TABLE_NAME);
        let plan = sqlx::query_as::<Sqlite, SQLEventEnvelope<SQLPrescriptionEvent>>(&query);
        let results = plan.fetch_all(&self.pool).await;
        match results {
            Err(e) => return Err(e.into()),
            _ => {}
        };
        let mut resp: Vec<EventEnvelope<PrescriptionAggregate>> = vec![];
        for env in results.unwrap() {
            let x = env.into();
            resp.push(x)
        }
        return Ok(resp);
    }
}
