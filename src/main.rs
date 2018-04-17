#[derive(Debug)]
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

const ANY_STRIKE: [Event; 7] = [
    Event::StrikeUnknownTypeCalledOrSwinging,
    Event::FoulBunt,
    Event::MissedBuntAttempt,
    Event::FoulTipOnBunt,
    Event::SwingingOnPitchout,
    Event::SwingingStrike,
    //  Does this count? Event::FoulTip,
    Event::CalledStrike,
];

const STRIKE_LESS_THAN_2: [Event; 10] = [
    Event::Foul,
    Event::StrikeUnknownTypeCalledOrSwinging,
    Event::FoulBunt,
    Event::MissedBuntAttempt,
    Event::FoulTipOnBunt,
    Event::SwingingOnPitchout,
    Event::FoulBallOnPitchout,
    Event::SwingingStrike,
    Event::FoulTip,
    Event::CalledStrike,
];

#[derive(Debug)]
enum Base {
    First,
    Second,
    Third,
}

#[derive(Debug)]
enum OffensivePlayerState {
    Bench,
    Batter { balls: u8, strikes: u8 },
    Runner { base: Base },
    Out,
    Failure(String),
}

impl OffensivePlayerState {
    fn next(self, event: Event) -> OffensivePlayerState {
        match self {
            OffensivePlayerState::Batter { strikes, balls } if strikes < 2 => match event {
 => OffensivePlayerState::Batter {
                    strikes: strikes + 1,
                    balls,
                },
                _ => 
            },
            _ => OffensivePlayerState::Failure(format!(
                "Invalid state, event combination: {:#?}, {:#?}",
                self, event
            )),
        }
    }
}

fn main() {
    let player_state = OffensivePlayerState::Batter {
        balls: 0,
        strikes: 0,
    };
    let player_state = player_state.next(Event::CalledStrike);
    println!("{:?}", player_state);
    let player_state = player_state.next(Event::CalledStrike);
    println!("{:?}", player_state);
    let player_state = player_state.next(Event::CalledStrike);
    println!("{:?}", player_state);
}
