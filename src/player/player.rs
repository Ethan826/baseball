use event::event::Event;
use traits::StateMachine;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Player {
    first_name: String,
    last_name: String,
    number: String,
}

impl<'a> StateMachine for Player {
    fn next(&self, _event: &Event) -> Player {
        self.clone()
    }
}
