use ulid::Ulid;

use crate::context::{
    common::domain::machine::State,
    prescription::domain::{
        entity::{
            command::{PrescriptionCommand, UpdatePrescriptionCommand},
            event::PrescriptionEvent,
        },
        machine::context::PrescriptionContext,
    },
};

pub struct Created;

impl<'a> State<PrescriptionContext<'a>> for Created {
    fn entry(&mut self, _context: &mut PrescriptionContext) {}

    fn exit(&mut self, context: &mut PrescriptionContext) {
        let command: &PrescriptionCommand = context.get_command().as_ref().unwrap();
        match command {
            PrescriptionCommand::UpdatePrescription(UpdatePrescriptionCommand { id, address }) => {
                context.set_event(PrescriptionEvent::PrescriptionUpdated {
                    address: address.clone(),
                    event_id: Ulid::new().to_string(),
                })
            }
            _ => {}
        }
    }

    fn update(&mut self, _context: &mut PrescriptionContext) {}
}
