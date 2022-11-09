use serde::{Deserialize, Serialize};

use crate::context::prescription::domain::entity::aggregate::PrescriptionAggregate;

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct RESTPrescriptionMutation {
    pub patient_id: Option<String>,
    pub medication_id: Option<String>,
    pub address: Option<String>,
}

impl RESTPrescriptionMutation {
    pub fn new(
        patient_id: Option<String>,
        medication_id: Option<String>,
        address: Option<String>,
    ) -> Self {
        return Self {
            patient_id,
            medication_id,
            address,
        };
    }
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct RESTPrescriptionQuery {
    pub id: Option<String>,
    pub patient_id: Option<String>,
    pub medication_id: Option<String>,
    pub address: Option<String>,
}

impl From<PrescriptionAggregate> for RESTPrescriptionQuery {
    fn from(value: PrescriptionAggregate) -> Self {
        return RESTPrescriptionQuery {
            id: value.id,
            patient_id: value.patient_id,
            medication_id: value.medication_id,
            address: value.address,
        };
    }
}
