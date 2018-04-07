use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "_T", deny_unknown_fields)]
pub enum Event {
    #[serde(rename_all = "camelCase")]
    LogCarePackageLand {
        item_package: ItemPackage,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogCarePackageSpawn {
        item_package: ItemPackage,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },

    #[serde(rename_all = "camelCase")]
    LogGameStatePeriodic {
        game_state: GameState,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },

    #[serde(rename_all = "camelCase")]
    LogItemAttach {
        character: Character,
        parent_item: Item,
        child_item: Item,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogItemDetach {
        character: Character,
        parent_item: Item,
        child_item: Item,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogItemDrop {
        character: Character,
        item: Item,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogItemEquip {
        character: Character,
        item: Item,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogItemPickup {
        character: Character,
        item: Item,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogItemUnequip {
        character: Character,
        item: Item,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogItemUse {
        character: Character,
        item: Item,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },

    #[serde(rename_all = "PascalCase")]
    LogMatchDefinition {
        match_id: String,
        ping_quality: String,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogMatchEnd {
        characters: Vec<Character>,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogMatchStart {
        map_name: String,
        weather_id: String,
        characters: Vec<Character>,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },

    #[serde(rename_all = "camelCase")]
    LogPlayerAttack {
        attack_id: i64,
        attacker: Character,
        attack_type: String,
        weapon: Item,
        vehicle: Vehicle,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogPlayerCreate {
        character: Character,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogPlayerKill {
        attack_id: i64,
        killer: Character,
        victim: Character,
        damage_type_category: String,
        damage_causer_name: String,
        distance: f64,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogPlayerLogin {
        result: bool,
        error_message: String,
        account_id: String,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogPlayerLogout {
        account_id: String,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogPlayerPosition {
        character: Character,
        elapsed_time: u16,
        num_alive_players: u8,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogPlayerTakeDamage {
        attack_id: i64,
        attacker: Character,
        victim: Character,
        damage_type_category: String,
        damage_reason: String,
        damage: f64,
        damage_causer_name: String,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },

    #[serde(rename_all = "camelCase")]
    LogVehicleDestroy {
        attack_id: i64,
        attacker: Character,
        vehicle: Vehicle,
        damage_type_category: String,
        damage_causer_name: String,
        distance: f64,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogVehicleLeave {
        character: Character,
        vehicle: Vehicle,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
    #[serde(rename_all = "camelCase")]
    LogVehicleRide {
        character: Character,
        vehicle: Vehicle,
        common: Common,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
        /// Update?
        #[serde(rename = "_U")]
        update: bool,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Common {
    match_id: String,
    map_name: String,
    is_game: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPackage {
    item_package_id: String,
    location: Location,
    items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    item_id: String,
    stack_count: i32,
    // TODO: Make enum
    category: String,
    sub_category: String,
    attached_items: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    name: String,
    team_id: u8,
    health: f64,
    location: Location,
    ranking: u8,
    account_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vehicle {
    vehicle_type: String,
    vehicle_id: String,
    health_percent: f64,
    feul_percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    elapsed_time: u16,
    num_alive_teams: u8,
    num_join_players: u8,
    num_start_players: u8,
    num_alive_players: u8,
    safety_zone_position: Location,
    safety_zone_radius: f64,
    poison_gas_warning_position: Location,
    poison_gas_warning_radius: f64,
    red_zone_position: Location,
    red_zone_radius: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    x: f64,
    y: f64,
    z: f64,
}
