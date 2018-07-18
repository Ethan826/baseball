/**
 * Baseball is fundamentally a stream of events. We should design the
 * architecture with the idea of FRP in mind, in which everyone listens to
 * events and the right actor acts accordingly. One of the complications of
 * that, however, is that to do loose coupling, we need to have entities able
 * to publish their own events. For example, a batter should know when he or
 * she has been retired or put on base, and should emit an event accordingly,
 * even if "Out Recorded" is not an event recognized explicitly by Retrosheet.
 */

#[derive(Debug, Clone)]
pub enum Event {
    // Plays initiated by pitcher
    PickoffThrowToFirst,
    PickoffThrowToSecond,
    PickoffThrowToThird,

    // Pitches
    Ball,
    CalledStrike,
    Foul,
    HitBatter,
    IntentionalBall,
    StrikeUnknownType,
    FoulBunt,
    MissedBuntAttempt,
    NoPitch,
    FoulTipOnBunt,
    Pitchout,
    SwingingOnPitchout,
    FoulBallOnPitchout,
    SwingingStrike,
    FoulTip,
    UnknownOrMissedPitch,
    CalledBallBecausePitcherWentToHisMouth,
    BallPutIntoPlayByBatter,
    BallPutIntoPlayOnPitchout,

    // Non at-bats
    Interference,
    IntentionalWalk,
    IntentionalWalkPlusEvent,
    Walk,
    WalkPlusEvent,
    BatterHitByPitch, // Distinct from the pitch event

    // Non-hits
    Putout,
    FieldersChoice,
    Error,
    FoulBallError, // No runners advance
    Strikeout,
    StrikeoutPlusEvent,

    // Hits
    Single,
    Double,
    Triple,
    HomeRun,
    GroundRuleDouble,

    // Baserunning events not involving batter
    Balk,
    CaughtStealing,
    DefensiveIndifference,
    PassedBall,
    WildPitch,
    Pickoff,
    PickoffCaughtStealing,
    StolenBase,
}
