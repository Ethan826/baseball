/**
 * The team namespace is responsible for the state of the team and for holding
 * the players in the game.
 */
use event::event::Event;
use player::player::Player;
use std::collections::HashSet;

use traits::StateMachine;

#[derive(Debug, Clone)]
pub struct Team {
    pub players: HashSet<Player>,
    pub lineup: Vec<Player>,
}

impl StateMachine for Team {
    fn next(&self, event: &Event) -> Team {
        Self {
            players: self.players.iter().map(|p| p.next(event)).collect(),
            lineup: self.lineup.iter().map(|p| p.next(event)).collect(),
        }
    }
}
