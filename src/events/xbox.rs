use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "_T", deny_unknown_fields)]
pub enum Event {
    #[serde(rename_all = "PascalCase")]
    LogCarePackageLand {
        item_package: ItemPackage,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogCarePackageSpawn {
        item_package: ItemPackage,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },

    #[serde(rename_all = "PascalCase")]
    LogGameStatePeriodic {
        game_state: GameState,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },

    #[serde(rename_all = "PascalCase")]
    LogItemAttach {
        character: Character,
        parent_item: Item,
        child_item: Item,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogItemDetach {
        character: Character,
        parent_item: Item,
        child_item: Item,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogItemDrop {
        character: Character,
        item: Item,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogItemEquip {
        character: Character,
        item: Item,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogItemPickup {
        character: Character,
        item: Item,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogItemUnequip {
        character: Character,
        item: Item,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogItemUse {
        character: Character,
        item: Item,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },

    #[serde(rename_all = "PascalCase")]
    LogMatchDefinition {
        match_id: String,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogMatchEnd {
        characters: Vec<Character>,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogMatchStart {
        characters: Vec<Character>,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },

    #[serde(rename_all = "PascalCase")]
    LogPlayerAttack {
        attack_id: i64,
        attacker: Character,
        attack_type: String,
        weapon: Item,
        vehicle: Vehicle,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogPlayerCreate {
        character: Character,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogPlayerKill {
        attack_id: i64,
        killer: Character,
        victim: Character,
        damage_type_category: String,
        damage_causer_name: String,
        distance: f64,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogPlayerLogin {
        result: bool,
        error_message: String,
        account_id: String,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogPlayerLogout {
        account_id: String,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogPlayerPosition {
        character: Character,
        elapsed_time: u16,
        num_alive_players: u8,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogPlayerTakeDamage {
        attack_id: i64,
        attacker: Character,
        victim: Character,
        damage_type_category: String,
        damage_reason: String,
        damage: f64,
        damage_causer_name: String,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },

    #[serde(rename_all = "PascalCase")]
    LogVehicleDestroy {
        attack_id: i64,
        attacker: Character,
        vehicle: Vehicle,
        damage_type_category: String,
        damage_causer_name: String,
        distance: f64,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogVehicleLeave {
        character: Character,
        vehicle: Vehicle,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
    #[serde(rename_all = "PascalCase")]
    LogVehicleRide {
        character: Character,
        vehicle: Vehicle,

        /// Version
        #[serde(rename = "_V")]
        version: u8,
        /// Event timestamp
        #[serde(rename = "_D")]
        datetime: DateTime<Utc>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemPackage {
    item_package_id: String,
    location: Location,
    items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    item_id: String,
    stack_count: i32,
    // TODO: Make enum
    category: String,
    sub_category: String,
    attached_items: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Character {
    name: String,
    team_id: u8,
    health: f64,
    location: Location,
    ranking: u8,
    account_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Vehicle {
    vehicle_type: String,
    vehicle_id: String,
    health_percent: f64,
    feul_percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
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
#[serde(rename_all = "PascalCase")]
pub struct Location {
    x: f64,
    y: f64,
    z: f64,
}
