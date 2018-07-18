use event::event::Event;
use game::inning_half::InningHalf;
use traits::StateMachine;

#[derive(Debug, Clone)]
pub struct Inning {
    pub inning_number: u8,
    pub inning_half: InningHalf,
}

impl StateMachine for Inning {
    fn next(&self, _event: &Event) -> Inning {
        self.clone()
    }
}
