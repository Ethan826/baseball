#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Event {
    PlayNotInvolvingTheBatter,
    PickoffThrowToFirst,
    PickoffThrowToSecond,
    PickoffThrowToThird,
    RunnerGoingOnPitch,
    Ball,
    CalledStrike,
    Foul,
    BatterHitByPitch,
    IntentionalBall,
    StrikeUnknownTypeCalledOrSwinging,
    FoulBunt,
    MissedBuntAttempt,
    NoPitchBalkOrInterference,
    FoulTipOnBunt,
    Pitchout,
    SwingingOnPitchout,
    FoulBallOnPitchout,
    SwingingStrike,
    FoulTip,
    Unknown,
    CalledBallBecausePitcherWentToHisMouth,
    BallPutInPlayByBatter,
    BallPutInPlayOnPitchout,
}

lazy_static! {
    static ref ANY_STRIKE: HashSet<Event> = [
        Event::CalledStrike,
        Event::FoulBunt,
        Event::FoulTip,
        Event::FoulTipOnBunt,
        Event::MissedBuntAttempt,
        Event::StrikeUnknownTypeCalledOrSwinging,
        Event::SwingingOnPitchout,
        Event::SwingingStrike,
    ].iter()
        .cloned()
        .collect();
    static ref STRIKE_LESS_THAN_2: HashSet<Event> = [
        Event::CalledStrike,
        Event::Foul,
        Event::FoulBallOnPitchout,
        Event::FoulBunt,
        Event::FoulTip,
        Event::FoulTipOnBunt,
        Event::MissedBuntAttempt,
        Event::StrikeUnknownTypeCalledOrSwinging,
        Event::SwingingOnPitchout,
        Event::SwingingStrike,
    ].iter()
        .cloned()
        .collect();
    static ref BALLS: HashSet<Event> = [
        Event::Ball,
        Event::IntentionalBall,
        Event::CalledBallBecausePitcherWentToHisMouth,
    ].iter()
        .cloned()
        .collect();
}

#[derive(Clone, Copy, Debug)]
enum Base {
    First,
    Second,
    Third,
}

#[derive(Clone, Debug)]
enum OffensivePlayerState {
    Bench,
    Batter { balls: u8, strikes: u8 },
    Runner { base: Base },
    Out,
    Failure(String),
}

impl OffensivePlayerState {
    fn next(self, event: Event) -> OffensivePlayerState {
        match (&self, event) {
            (OffensivePlayerState::Batter { strikes, balls }, _)
                if *strikes < 2 && STRIKE_LESS_THAN_2.contains(&event) =>
            {
                OffensivePlayerState::Batter {
                    balls: *balls,
                    strikes: strikes + 1,
                }
            }
            (OffensivePlayerState::Batter { strikes, .. }, _)
                if *strikes == 2 && ANY_STRIKE.contains(&event) =>
            {
                OffensivePlayerState::Out
            }
            (OffensivePlayerState::Batter { strikes, balls }, _)
                if *balls < 3 && BALLS.contains(&event) =>
            {
                OffensivePlayerState::Batter {
                    strikes: *strikes,
                    balls: balls + 1,
                }
            }
            (OffensivePlayerState::Batter { balls, .. }, _)
                if *balls == 3 && BALLS.contains(&event) =>
            {
                OffensivePlayerState::Runner { base: Base::First }
            }
            _ => self.clone()
            // Failure state for non-batting plays with batter
            // _ => OffensivePlayerState::Failure(format!(
            //     "Invalid state, event combination: {:?}, {:?}",
            //     self, event
            // )),
        }
    }
}

fn main() {
    let player_state = OffensivePlayerState::Batter {
        balls: 0,
        strikes: 0,
    };
    println!("{:?}", player_state);
    let player_state = player_state.next(Event::CalledStrike);
    println!("{:?}", player_state);
    let player_state = player_state.next(Event::Ball);
    println!("{:?}", player_state);
    let player_state = player_state.next(Event::Foul);
    println!("{:?}", player_state);
    let player_state = player_state.next(Event::Ball);
    println!("{:?}", player_state);
    let player_state = player_state.next(Event::Ball);
    println!("{:?}", player_state);
    let player_state = player_state.next(Event::Foul);
    println!("{:?}", player_state);
    let player_state = player_state.next(Event::Ball);
    println!("{:?}", player_state);
}
