use crate::context::prescription::application::ports::outbound::prescription::PrescriptionServices;
use crate::context::prescription::domain::entity::{
    command::PrescriptionCommand, event::PrescriptionEvent,
};

#[derive(Debug)]
pub struct PrescriptionContext<'a> {
    command: Option<PrescriptionCommand>,
    event: Option<PrescriptionEvent>,
    _services: &'a Box<dyn PrescriptionServices + Send + Sync>,
}

impl<'a> PrescriptionContext<'a> {
    pub fn new(services: &'a Box<dyn PrescriptionServices + Send + Sync>) -> Self {
        return Self {
            command: None,
            event: None,
            _services: services,
        };
    }
    pub fn get_event(&self) -> &Option<PrescriptionEvent> {
        return &self.event;
    }
    pub fn set_event(&mut self, event: PrescriptionEvent) {
        self.event = Some(event);
    }
    pub fn get_command(&self) -> &Option<PrescriptionCommand> {
        return &self.command;
    }
    pub fn set_command(&mut self, command: PrescriptionCommand) {
        self.command = Some(command);
    }
}
