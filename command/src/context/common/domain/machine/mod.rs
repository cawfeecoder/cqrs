use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub type Condition<T> = fn(&T) -> bool;

pub type Action<T> = fn(&mut T) -> ();
pub trait State<T>
where
    T: Debug,
{
    fn entry(&mut self, _context: &mut T) {}
    fn exit(&mut self, _context: &mut T) {}
    fn update(&mut self, _context: &mut T) {}
}

pub struct FSMTransition<K, T> {
    to: K,
    condition: Condition<T>,
    actions: Vec<Action<T>>,
}

pub struct FSMState<K, T> {
    state: Box<dyn State<T>>,
    transitions: Vec<FSMTransition<K, T>>,
}

impl<K, T> FSMState<K, T>
where
    T: Debug,
{
    pub fn new<S: State<T> + 'static>(state: S) -> Self {
        Self {
            state: Box::new(state),
            transitions: vec![],
        }
    }

    pub fn transition(mut self, to: K, condition: Condition<T>, actions: Vec<Action<T>>) -> Self {
        self.transitions.push(FSMTransition {
            to,
            condition,
            actions,
        });
        self
    }

    fn decide(&self, context: &mut T) -> Option<&FSMTransition<K, T>>
    where
        K: Clone,
    {
        for transition in self.transitions.iter() {
            if (transition.condition)(context) {
                return Some(transition);
            }
        }
        None
    }
}

pub struct FSM<K, T> {
    states: HashMap<K, FSMState<K, T>>,
    active_state: K,
}

impl<K: Hash + Eq + Debug, T: Debug> FSM<K, T> {
    pub fn new(active_state: K) -> Self {
        Self {
            states: Default::default(),
            active_state,
        }
    }

    pub fn state(mut self, id: K, state: FSMState<K, T>) -> Self {
        self.states.insert(id, state);
        self
    }

    pub fn set_active_state(&mut self, id: K, actions: Vec<Action<T>>, context: &mut T) {
        if let Some(state) = self.states.get_mut(&self.active_state) {
            state.state.exit(context);
        }
        for action in actions {
            (action)(context)
        }
        if let Some(state) = self.states.get_mut(&id) {
            state.state.entry(context);
            self.active_state = id;
        }
    }

    pub fn decide(&mut self, context: &mut T)
    where
        K: Clone,
    {
        if let Some(state) = self.states.get(&self.active_state) {
            if let Some(selected) = state.decide(context) {
                self.set_active_state(selected.to.clone(), selected.actions.clone(), context);
            }
        }
    }

    pub fn update(&mut self, context: &mut T) {
        if let Some(state) = self.states.get_mut(&self.active_state) {
            state.state.update(context);
        }
    }
}
