use event::event::Event;
use traits::StateMachine;

#[derive(Debug, Clone)]
pub enum InningHalf {
    Top,
    Bottom,
}

impl StateMachine for InningHalf {
    fn next(&self, _event: &Event) -> InningHalf {
        self.clone()
    }
}
