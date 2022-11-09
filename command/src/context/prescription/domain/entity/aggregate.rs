use async_trait::async_trait;
use chrono::Utc;
use ulid::Ulid;

use crate::context::{
    common::domain::entity::{
        aggregate::Aggregate,
        event::{AggregateSnapshot, DomainEvent},
    },
    prescription::{
        application::ports::outbound::prescription::PrescriptionServices,
        domain::machine::{
            context::PrescriptionContext, create_prescription_machine, states::States,
        },
    },
};

use super::{command::PrescriptionCommand, error::PrescriptionError, event::PrescriptionEvent};

#[derive(Clone, Debug)]
pub struct PrescriptionAggregate {
    pub id: Option<String>,
    pub patient_id: Option<String>,
    pub medication_id: Option<String>,
    pub address: Option<String>,
    pub last_event: Option<PrescriptionEvent>,
    pub applied_events: i32,
}

#[async_trait]
impl Aggregate for PrescriptionAggregate {
    type Command = PrescriptionCommand;
    type Event = PrescriptionEvent;
    type Error = PrescriptionError;
    type Services = Box<dyn Sync + Send + PrescriptionServices>;

    fn aggregate_type() -> String {
        "Prescription".to_string()
    }

    fn aggregate_id(&self) -> Option<String> {
        return self.id.clone();
    }

    async fn handle(
        &self,
        command: Self::Command,
        services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        let mut fsm = match &self.last_event {
            Some(x) => match x {
                PrescriptionEvent::PrescriptionCreated { .. } => {
                    create_prescription_machine(States::Created)
                }
                PrescriptionEvent::PrescriptionUpdated { .. } => {
                    create_prescription_machine(States::Created)
                }
            },
            None => create_prescription_machine(States::New),
        };
        let mut context: PrescriptionContext = PrescriptionContext::new(services);
        context.set_command(command.clone());
        fsm.decide(&mut context);
        return match context.get_event() {
            Some(x) => Ok(vec![x.clone()]),
            None => Err(PrescriptionError::StateMachineTransitionFail(
                command.to_string(),
            )),
        };
    }

    fn apply(&mut self, event: Self::Event) {
        self.applied_events += 1;
        match &event {
            PrescriptionEvent::PrescriptionCreated {
                id,
                patient_id,
                medication_id,
                address,
                ..
            } => {
                self.id = Some(id.clone());
                self.medication_id = Some(medication_id.clone());
                self.patient_id = Some(patient_id.clone());
                self.address = Some(address.clone());
                self.last_event = Some(event);
            }
            PrescriptionEvent::PrescriptionUpdated { address, .. } => {
                self.address = Some(address.clone());
                self.last_event = Some(event);
            }
        }
    }

    fn snapshot(&mut self) -> Option<AggregateSnapshot<Self>> {
        if self.applied_events >= 10 {
            let snapshot: AggregateSnapshot<Self> = AggregateSnapshot {
                aggregate_id: self.aggregate_id().unwrap(),
                aggregate_type: Self::aggregate_type(),
                payload: self.clone(),
                last_sequence: self.last_event.as_ref().unwrap().event_id(),
                snapshot_id: Ulid::new().to_string(),
                timestamp: Utc::now(),
            };
            return Some(snapshot);
        }
        return None;
    }
}

impl Default for PrescriptionAggregate {
    fn default() -> Self {
        PrescriptionAggregate {
            id: None,
            patient_id: None,
            medication_id: None,
            address: None,
            last_event: None,
            applied_events: 0,
        }
    }
}

#[cfg(test)]
mod prescription_test {
    use crate::context::common::domain::entity::aggregate::Aggregate;
    use crate::context::common::domain::entity::event::DomainEvent;
    use crate::context::prescription::application::ports::outbound::prescription::{
        MockPrescriptionServices, PrescriptionServices,
    };
    use crate::context::prescription::domain::entity::command::CreatePrescriptionCommand;
    use crate::context::prescription::domain::entity::{
        aggregate::PrescriptionAggregate, command::PrescriptionCommand,
    };

    #[tokio::test]
    async fn emit_prescription_created_event_when_receive_valid_create_prescription_command() {
        let expected = "PrescriptionCreated";

        let aggregate = PrescriptionAggregate::default();

        let command = PrescriptionCommand::CreatePrescription(CreatePrescriptionCommand {
            medication_id: "1234".into(),
            patient_id: "1234".into(),
            address: "1234".into(),
        });

        let mock: Box<dyn PrescriptionServices + Sync + Send> =
            Box::new(MockPrescriptionServices::new());

        let events = aggregate.handle(command, &mock).await;

        assert!(events.is_ok());
        let unwrapped = events.unwrap();
        assert_eq!(unwrapped.len(), 1);
        assert_eq!(unwrapped[0].event_type(), expected);
    }
}
