/**
 * The game namespace is responsible for the state of the game and for holding
 * the teams playing the game.
 */
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

#[derive(Debug, Clone)]
pub struct Game {
    pub home_team: Team,
    pub visiting_team: Team,
    pub inning: Inning,
    pub out: Out,
}

impl StateMachine for Game {
    fn next(&self, event: &Event) -> Game {
        Self {
            home_team: self.home_team.next(event),
            visiting_team: self.visiting_team.next(event),
            inning: self.inning.next(event),
            out: self.out.next(event),
        }
    }
}

impl StateMachine for Out {
    fn next(&self, _event: &Event) -> Out {
        self.clone()
    }
}
