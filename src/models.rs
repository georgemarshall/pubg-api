#![allow(unused_variables)]
use std::fmt::Display;
use std::str::FromStr;
use std::time::Duration;

use chrono::prelude::*;
use serde::de::{self, Deserialize, Deserializer};
use serde_json::Number;

use json_uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Asset {
    id: Uuid,
    /// URL
    #[serde(rename = "-u-r-l")]
    url: String,
    /// createdAt
    created_at: DateTime<Utc>,
    /// description
    description: String,
    /// name
    name: String,
}

resource!(Asset, |&self| {
    kind "asset";
    id self.id;
    attrs url, created_at, description, name;
});

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Match {
    id: Uuid,
    /// createdAt
    created_at: DateTime<Utc>,
    /// Duration
    #[serde(deserialize_with = "from_secs")]
    duration: Duration,
    /// gameMode
    game_mode: String,
    /// patchVersion
    patch_version: String,
    /// shardId
    shard_id: PlatformRegion,
    /// stats - undocumented
    stats: Option<MatchStats>,
    /// tags
    // FIXME: Actual structure unknown
    tags: Option<String>,
    /// titleId
    title_id: String,

    // relationships
    assets: Vec<Asset>,
    rosters: Vec<Roster>,
    rounds: Vec<Round>,
    spectators: Vec<Spectator>,
}
resource!(Match, |&self| {
    kind "match";
    id self.id;
    attrs created_at, duration, game_mode, patch_version, shard_id, tags, title_id;
    has_many assets, rosters, rounds, spectators;
});

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Participant {
    id: Uuid,
    /// actor
    actor: String,
    /// shardId
    shard_id: PlatformRegion,
    /// stats
    stats: ParticipantStats,
}
resource!(Participant, |&self| {
    kind "participant";
    id self.id;
    attrs actor, shard_id, stats;
});

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Roster {
    id: Uuid,
    /// shardId
    shard_id: PlatformRegion,
    /// stats
    stats: RosterStats,
    /// Won
    #[serde(deserialize_with = "from_str")]
    won: bool,

    // relationships
    participants: Vec<Participant>,
    team: Option<Team>,
}
resource!(Roster, |&self| {
    kind "roster";
    id self.id;
    attrs shard_id, stats, won;
    has_one team;
    has_many participants;
});

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Status {
    id: String,
    /// releasedAt
    released_at: DateTime<Utc>,
    /// version
    version: String,
}
resource!(Status, |&self| {
    kind "status";
    id self.id;
    attrs released_at, version;
});

//////////////////////
// Supporting Types //
//////////////////////

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeathType {
    Alive,
    Byplayer,
    Logout,
    Suicide,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct MatchStats {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ParticipantStats {
    /// DBNOs - Dead, But Not Outs
    #[serde(rename = "-d-b-n-os")]
    dbnos: u8,
    /// assists
    assists: u8,
    /// boosts
    boosts: u8,
    /// damageDealt
    damage_dealt: f64,
    /// deathType
    death_type: DeathType,
    /// headshotKills
    headshot_kills: u8,
    /// heals
    heals: u8,
    /// killPlace
    kill_place: u8,
    /// killPoints - undocumented
    kill_points: Option<u16>,
    /// killPointsDelta
    kill_points_delta: f64,
    /// killStreaks
    kill_streaks: u8,
    /// kills
    kills: u8,
    /// lastKillPoints
    last_kill_points: u16,
    /// lastWinPoints
    last_win_points: u16,
    /// longestKill
    longest_kill: f64,
    /// mostDamage
    most_damage: u16,
    /// name
    name: String,
    /// playerId
    player_id: String,
    /// revives
    revives: u8,
    /// rideDistance
    ride_distance: f64,
    /// roadKills
    road_kills: u8,
    /// teamKills
    team_kills: u8,
    /// timeSurvived - possibly still f64
    #[serde(deserialize_with = "from_secs")]
    time_survived: Duration,
    /// vehicleDestroys
    vehicle_destroys: u8,
    /// walkDistance
    walk_distance: f64,
    /// weaponsAcquired
    weapons_acquired: u8,
    /// winPlace
    win_place: u8,
    /// winPoints - undocumented
    win_points: Option<u16>,
    /// winPointsDelta
    win_points_delta: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlatformRegion {
    XboxAs,
    XboxEu,
    XboxNa,
    XboxOc,
    PcKrjp,
    PcNa,
    PcEu,
    PcOc,
    PcKakao,
    PcSea,
    PcSa,
    PcAs,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct RosterStats {
    rank: u8,
    team_id: u8,
}

/////////////////
// Other Types //
/////////////////

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Player {
    id: Uuid,
    name: String,
    patch_version: String,
    shard_id: PlatformRegion,
    stats: String,
    title_id: String,
    assets: Vec<Asset>,
}
resource!(Player, |&self| {
    kind "player";
    id self.id;
    attrs name, patch_version, shard_id, stats, title_id;
    has_many assets;
});

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Round {
    id: Uuid,
}
resource!(Round, |&self| {
    kind "round";
    id self.id;
});

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Spectator {
    id: Uuid,
}
resource!(Spectator, |&self| {
    kind "spectator";
    id self.id;
});

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Team {
    id: Uuid,
}
resource!(Team, |&self| {
    kind "team";
    id self.id;
});

fn from_secs<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let num = Number::deserialize(deserializer)?;
    num.as_u64()
        .map(Duration::from_secs)
        .ok_or_else(|| de::Error::custom("Bad duration value"))
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}
