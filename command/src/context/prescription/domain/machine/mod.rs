use crate::context::common::domain::machine::{FSMState, FSM};

use self::{
    context::PrescriptionContext,
    states::{created::Created, new::New, States},
};

pub mod context;
pub mod states;

pub type PrescriptionMachine<'a> = FSM<States, PrescriptionContext<'a>>;

pub fn create_prescription_machine<'a>(initial_state: States) -> PrescriptionMachine<'a> {
    let fsm = FSM::new(initial_state)
        .state(
            States::New,
            FSMState::new(New).transition(
                States::Created,
                |data| {
                    return data.get_command().is_some()
                        && data.get_command().as_ref().unwrap().to_string()
                            == "CreatePrescription";
                },
                vec![],
            ),
        )
        .state(
            States::Created,
            FSMState::new(Created).transition(
                States::Created,
                |data| {
                    return data.get_command().is_some()
                        && data.get_command().as_ref().unwrap().to_string()
                            == "UpdatePrescription";
                },
                vec![],
            ),
        );
    return fsm;
}
