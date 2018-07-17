/**.
 * The team namespace is responsible for the state of the team and for holding
 * the players in the game.
 */
use std::collections::HashSet;

use player::Player;

pub struct Team {
    pub players: HashSet<Player>,
    pub lineup: Vec<Player>,
}
