#[derive(Debug, Clone)]
pub enum PrescriptionCommand {
    CreatePrescription(CreatePrescriptionCommand),
    UpdatePrescription(UpdatePrescriptionCommand),
}

impl PrescriptionCommand {
    pub fn to_string(&self) -> String {
        match self {
            Self::CreatePrescription { .. } => "CreatePrescription".into(),
            Self::UpdatePrescription { .. } => "UpdatePrescription".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreatePrescriptionCommand {
    pub medication_id: String,
    pub patient_id: String,
    pub address: String,
}

impl Into<PrescriptionCommand> for CreatePrescriptionCommand {
    fn into(self) -> PrescriptionCommand {
        PrescriptionCommand::CreatePrescription(self)
    }
}

#[derive(Debug, Clone)]
pub struct UpdatePrescriptionCommand {
    pub id: String,
    pub address: String,
}

impl Into<PrescriptionCommand> for UpdatePrescriptionCommand {
    fn into(self) -> PrescriptionCommand {
        PrescriptionCommand::UpdatePrescription(self)
    }
}
