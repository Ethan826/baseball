/**
 * The team namespace is responsible for the state of the team and for holding
 * the players in the game.
 */
use std::collections::HashSet;

use event::event::Event;
use player::player::Player;
use traits::StateMachine;

#[derive(Debug, Clone)]
pub struct Team<'a> {
    pub roster: HashSet<&'a Player>,
    pub lineup: Vec<usize>, // Indices into roster
    pub current_batter: usize,
}

impl<'a> StateMachine for Team<'a> {
    fn next(&self, event: &Event) -> Team<'a> {
        Team {
            roster: self.roster.iter().map(|p| p.next(event)).collect(),
            lineup: self.lineup.iter().map(|p| p.next(event)).collect(),
        }
    }
}

// =================================================================================================
// Test helpers
// =================================================================================================

#[cfg(test)]
impl Team {
    fn base() {
        Team {
            players: (0..25).map(|_| Player::base()).collect(),
            lineup,
        }
    }
}
