//! The game namespace is responsible for the state of the game and for holding
//! the teams playing the game.

use event::event::Event;
use game::inning::Inning;
use team::team::Team;
use traits::StateMachine;

#[derive(Debug, Clone)]
pub enum Out {
    One,
    Two,
    Three,
}

#[derive(Clone, Debug)]
pub struct Game<'a> {
    pub home_team: &'a mut Team<'a>,
    pub visiting_team: &'a mut Team<'a>,
    pub inning: Inning,
    pub out: Out,
}

impl<'a> StateMachine for Game<'a> {
    fn next(&self, _event: &Event) -> Game<'a> {
        self.clone()
    }
}

impl StateMachine for Out {
    fn next(&self, _event: &Event) -> Out {
        self.clone()
    }
}

// =================================================================================================
// Test helpers
// =================================================================================================

#[cfg(test)]
impl Game {
    fn base() -> Game {
        Game {
            home_team: Team::base(),
            visiting_team: Team::base(),
            inning: Inning::base(),
            out: Out::base(),
        }
    }
}
