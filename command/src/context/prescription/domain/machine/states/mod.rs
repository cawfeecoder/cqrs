pub mod created;
pub mod new;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum States {
    Created,
    New,
}
