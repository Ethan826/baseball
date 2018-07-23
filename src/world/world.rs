use game::game::Game;
use player::player::Player;
use std::collections::HashMap;
use team::team::Team;

pub type Id = String;

// TODO: World can only have one game at a time in this configuration, which is
// to see if we can have game have mutable references to the two teams (that are
// owned by world). Is there a way to have multiple concurrent games existing in
// the world, where each team is uniquely owned by at most one extant game? This
// seems like a runtime vs compile-time issue, so probably an Rc or Mutex or
// something?
pub struct World<'a> {
    pub players: HashMap<Id, Player>,
    pub teams: HashMap<Id, Team<'a>>,
    pub game: Game<'a>,
}
