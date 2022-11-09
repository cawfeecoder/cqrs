pub mod context;

use std::sync::Arc;

use anyhow::anyhow;
use context::common::application::ports::outbound::event_bus::EventBus;
use context::common::domain::entity::event::EventEnvelope;
use sqlx::{Pool, Sqlite};

use crate::context::common::infrastructure::adapters::secondary::eventbus::channel::ChannelBus;
use crate::context::common::infrastructure::adapters::secondary::storage::sqlite::SqliteConnector;
use crate::context::prescription::application::ports::inbound::get_events::GetEvents;
use crate::context::prescription::application::ports::inbound::send_event::SendEvent;
use crate::context::prescription::application::ports::outbound::prescription::MockPrescriptionServices;
use crate::context::prescription::application::ports::outbound::prescription::PrescriptionServices;
use crate::context::prescription::application::service::outbox::PrescriptionOutboxService;
use crate::context::prescription::application::service::prescription::PrescriptionService;
use crate::context::prescription::domain::entity::aggregate::PrescriptionAggregate;
use crate::context::prescription::infrastructure::adapters::primary::rest::RESTPrescriptionAdapter;

use tokio::signal;
use tokio::signal::unix::signal;
use tokio::signal::unix::SignalKind;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    //TODO: load aggregate from snapshots + events
    //TODO: call handle to generate events
    //TODO: commit events and then dispatch events
    let conn: Result<Pool<Sqlite>, anyhow::Error> = sqlx::Pool::connect("sqlite://test.db")
        .await
        .map_err(|e| anyhow!(e));
    let connector = SqliteConnector::new(conn).await.unwrap();
    let services: Box<dyn PrescriptionServices + Sync + Send> =
        Box::new(MockPrescriptionServices::new());
    let service: Arc<PrescriptionService> =
        Arc::new(PrescriptionService::new(services, connector.clone()));

    let eventbus: ChannelBus<EventEnvelope<PrescriptionAggregate>> = ChannelBus::new();
    let receiver = eventbus.receive_events();

    let outbox_service: Arc<PrescriptionOutboxService> = Arc::new(PrescriptionOutboxService::new(
        connector.clone(),
        Arc::new(eventbus),
    ));

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));

        loop {
            interval.tick().await;
            match outbox_service.get_events().await {
                Ok(x) => {
                    if x.len() == 0 {
                        println!("No events in outbox queue");
                    }
                    for event in x {
                        match outbox_service.send_event(event).await {
                            Err(e) => println!("Received error sending event: {:?}", e),
                            _ => (),
                        }
                    }
                }
                Err(e) => println!("Received Error: {:?}", e),
            }
        }
    });

    tokio::spawn(async move {
        let rest = RESTPrescriptionAdapter::new(service);
        rest.run().await;
    });

    let mut sigterm = signal(SignalKind::terminate())?;

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("🎩 Ctrl-C received, shutting down");
        }
        _ = sigterm.recv() => {
            println!("terminate signal received, shutting down");
        }
    }

    Ok(())
}
