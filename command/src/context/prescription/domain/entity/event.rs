use ulid::Ulid;

use crate::context::common::domain::entity::event::DomainEvent;

#[derive(Debug, Clone, PartialEq)]
pub enum PrescriptionEvent {
    PrescriptionCreated {
        id: String,
        patient_id: String,
        medication_id: String,
        address: String,
        event_id: String,
    },
    PrescriptionUpdated {
        address: String,
        event_id: String,
    },
}

impl DomainEvent for PrescriptionEvent {
    fn event_type(&self) -> String {
        match self {
            PrescriptionEvent::PrescriptionCreated { .. } => "PrescriptionCreated".into(),
            PrescriptionEvent::PrescriptionUpdated { .. } => "PrescriptionUpdated".into(),
        }
    }
    fn event_version(&self) -> String {
        return "0.0.1".into();
    }
    fn event_id(&self) -> String {
        match self {
            PrescriptionEvent::PrescriptionCreated { event_id, .. } => event_id.clone(),
            PrescriptionEvent::PrescriptionUpdated { event_id, .. } => event_id.clone(),
        }
    }
}
