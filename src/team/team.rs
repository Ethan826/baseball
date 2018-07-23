//! The team namespace is responsible for the state of the team and for holding
//!  the players in the game.

use std::collections::BTreeSet;
use world::world::Id;

use event::event::Event;
use traits::StateMachine;

#[derive(Debug, Clone, Hash)]
pub struct Team<'a> {
    pub roster: BTreeSet<&'a Id>,
    pub lineup: BTreeSet<&'a Id>,
    pub current_batter: &'a Id,
}

impl<'a> StateMachine for Team<'a> {
    fn next(&self, _event: &Event) -> Team<'a> {
        self.clone()
    }
}

// =================================================================================================
// Test helpers
// =================================================================================================

#[cfg(test)]
impl<'a> Team<'a> {
    fn base() {}
}
