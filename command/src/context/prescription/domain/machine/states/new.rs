use crate::context::prescription::domain::entity::command::{
    CreatePrescriptionCommand, PrescriptionCommand,
};
use crate::context::{
    common::domain::machine::State, prescription::domain::machine::context::PrescriptionContext,
};

use crate::context::prescription::domain::entity::event::PrescriptionEvent;

use ulid::Ulid;

pub struct New;

impl<'a> State<PrescriptionContext<'a>> for New {
    fn entry(&mut self, _context: &mut PrescriptionContext) {}

    fn exit(&mut self, context: &mut PrescriptionContext) {
        let command: &PrescriptionCommand = context.get_command().as_ref().unwrap();
        match command {
            PrescriptionCommand::CreatePrescription(CreatePrescriptionCommand {
                medication_id,
                patient_id,
                address,
            }) => context.set_event(PrescriptionEvent::PrescriptionCreated {
                id: Ulid::new().to_string(),
                medication_id: medication_id.clone(),
                patient_id: patient_id.clone(),
                address: address.clone(),
                event_id: Ulid::new().to_string(),
            }),
            _ => {}
        }
    }

    fn update(&mut self, _context: &mut PrescriptionContext) {}
}
