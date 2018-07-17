/**
 * The game namespace is responsible for the state of the game and for holding
 * the teams playing the game.
 */
use team::Team;

enum InningHalf {
    Top,
    Bottom,
}

struct Inning {
    inning_number: u8,
    inning_half: InningHalf,
}

enum Out {
    One,
    Two,
    Three,
}

struct Game {
    home_team: Team,
    visiting_team: Team,
    inning: Inning,
    out: Out,
}
