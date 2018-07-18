use event::event::Event;

pub trait StateMachine {
    fn next(&self, &Event) -> Self;
}
