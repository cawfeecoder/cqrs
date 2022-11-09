use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::context::prescription::domain::entity::{
    aggregate::PrescriptionAggregate, event::PrescriptionEvent,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event_type")]
pub enum SQLPrescriptionEvent {
    PrescriptionCreated {
        id: String,
        event_id: String,
        medication_id: String,
        patient_id: String,
        address: String,
    },
    PrescriptionUpdated {
        event_id: String,
        address: String,
    },
}

impl Default for SQLPrescriptionEvent {
    fn default() -> Self {
        return Self::PrescriptionCreated {
            id: "".into(),
            event_id: Ulid::new().to_string(),
            medication_id: "".into(),
            patient_id: "".into(),
            address: "".into(),
        };
    }
}

impl Into<Option<PrescriptionEvent>> for SQLPrescriptionEvent {
    fn into(self) -> Option<PrescriptionEvent> {
        match self {
            Self::PrescriptionCreated {
                id,
                event_id,
                patient_id,
                medication_id,
                address,
            } => Some(PrescriptionEvent::PrescriptionCreated {
                id,
                event_id,
                patient_id,
                medication_id,
                address,
            }),
            Self::PrescriptionUpdated { event_id, address } => {
                Some(PrescriptionEvent::PrescriptionUpdated { address, event_id })
            }
        }
    }
}

impl From<PrescriptionEvent> for SQLPrescriptionEvent {
    fn from(u: PrescriptionEvent) -> Self {
        match u {
            PrescriptionEvent::PrescriptionCreated {
                id,
                event_id,
                medication_id,
                patient_id,
                address,
            } => Self::PrescriptionCreated {
                id,
                event_id,
                medication_id,
                patient_id,
                address,
            },
            PrescriptionEvent::PrescriptionUpdated { address, event_id } => {
                Self::PrescriptionUpdated { event_id, address }
            }
        }
    }
}

impl Into<PrescriptionEvent> for SQLPrescriptionEvent {
    fn into(self) -> PrescriptionEvent {
        match self {
            Self::PrescriptionCreated {
                id,
                event_id,
                patient_id,
                medication_id,
                address,
            } => PrescriptionEvent::PrescriptionCreated {
                id,
                event_id,
                medication_id,
                patient_id,
                address,
            },
            Self::PrescriptionUpdated { event_id, address } => {
                PrescriptionEvent::PrescriptionUpdated { address, event_id }
            }
        }
    }
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct SQLPrescriptionAggregate {
    id: Option<String>,
    patient_id: Option<String>,
    medication_id: Option<String>,
    last_event: Option<SQLPrescriptionEvent>,
}

impl Into<PrescriptionAggregate> for SQLPrescriptionAggregate {
    fn into(self) -> PrescriptionAggregate {
        return PrescriptionAggregate {
            id: self.id,
            patient_id: self.patient_id,
            medication_id: self.medication_id,
            last_event: self.last_event.map(|x| x.into()),
            ..Default::default()
        };
    }
}

impl From<PrescriptionAggregate> for SQLPrescriptionAggregate {
    fn from(value: PrescriptionAggregate) -> Self {
        return SQLPrescriptionAggregate {
            id: value.id,
            patient_id: value.patient_id,
            medication_id: value.medication_id,
            last_event: value.last_event.map(|x| x.into()),
        };
    }
}
