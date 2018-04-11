use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum DamageReason {
    ArmShot,
    HeadShot,
    LegShot,
    None,
    NonSpecific,
    PelvisShot,
    TorsoShot,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DamageCauserName {
    #[serde(rename = "BattleRoyaleModeController_Def_C")]
    BattleRoyaleModeControllerDef,
    #[serde(rename = "BattleRoyaleModeController_Desert_C")]
    BattleRoyaleModeControllerDesert,

    #[serde(rename = "Buff_DecreaseBreathInApnea_C")]
    BuffDecreaseBreathInApnea,
    #[serde(rename = "Buff_FireDOT_C")]
    BuffFireDOT,

    // Vehicles
    #[serde(rename = "Boat_PG117_C")]
    Pg117,
    #[serde(rename = "BP_Motorbike_04_C")]
    Motorbike,
    #[serde(rename = "BP_Motorbike_04_Desert_C")]
    MotorbikeDesert,
    #[serde(rename = "BP_Motorbike_04_SideCar_C")]
    MotorbikeSideCar,
    #[serde(rename = "BP_Motorbike_04_SideCar_Desert_C")]
    MotorbikeSideCarDesert,
    #[serde(rename = "BP_PickupTruck_A_01_C")]
    PickupTruckA1,
    #[serde(rename = "BP_PickupTruck_A_02_C")]
    PickupTruckA2,
    #[serde(rename = "BP_PickupTruck_A_03_C")]
    PickupTruckA3,
    #[serde(rename = "BP_PickupTruck_A_04_C")]
    PickupTruckA4,
    #[serde(rename = "BP_PickupTruck_A_05_C")]
    PickupTruckA5,
    #[serde(rename = "BP_PickupTruck_B_01_C")]
    PickupTruckB1,
    #[serde(rename = "BP_PickupTruck_B_02_C")]
    PickupTruckB2,
    #[serde(rename = "BP_PickupTruck_B_03_C")]
    PickupTruckB3,
    #[serde(rename = "BP_PickupTruck_B_04_C")]
    PickupTruckB4,
    #[serde(rename = "BP_PickupTruck_B_05_C")]
    PickupTruckB5,
    #[serde(rename = "BP_Van_A_01_C")]
    VanA1,
    #[serde(rename = "BP_Van_A_02_C")]
    VanA2,
    #[serde(rename = "BP_Van_A_03_C")]
    VanA3,
    #[serde(rename = "Buggy_A_01_C")]
    BuggyA1,
    #[serde(rename = "Buggy_A_02_C")]
    BuggyA2,
    #[serde(rename = "Buggy_A_03_C")]
    BuggyA3,
    #[serde(rename = "Buggy_A_04_C")]
    BuggyA4,
    #[serde(rename = "Buggy_A_05_C")]
    BuggyA5,
    #[serde(rename = "Buggy_A_06_C")]
    BuggyA6,
    #[serde(rename = "Dacia_A_01_v2_C")]
    DaciaA1,
    #[serde(rename = "Dacia_A_02_v2_C")]
    DaciaA2,
    #[serde(rename = "Dacia_A_03_v2_C")]
    DaciaA3,
    #[serde(rename = "Dacia_A_04_v2_C")]
    DaciaA4,
    #[serde(rename = "PlayerFemale_A_C")]
    PlayerFemale,
    #[serde(rename = "PlayerMale_A_C")]
    PlayerMale,
    #[serde(rename = "ProjGrenade_C")]
    ProjGrenade,
    #[serde(rename = "ProjMolotov_C")]
    ProjMolotov,
    #[serde(rename = "ProjMolotov_DamageField_C")]
    ProjMolotovDamageField,
    #[serde(rename = "ProjMolotov_DamageFieldInWall_C")]
    ProjMolotovDamageFieldInWall,
    #[serde(rename = "RedZoneBomb_C")]
    RedZoneBomb,
    #[serde(rename = "Uaz_A_01_C")]
    UazA1,
    #[serde(rename = "Uaz_B_01_C")]
    UazB1,
    #[serde(rename = "Uaz_C_01_C")]
    UazC1,

    // Weapons
    #[serde(rename = "WeapAK47_C")]
    WeapAK47,
    #[serde(rename = "WeapAUG_C")]
    WeapAUG,
    #[serde(rename = "WeapAWM_C")]
    WeapAWM,
    #[serde(rename = "WeapBerreta686_C")]
    WeapBerreta686,
    #[serde(rename = "WeapCowbar_C")]
    WeapCowbar,
    #[serde(rename = "WeapCrossbow_1_C")]
    WeapCrossbow,
    #[serde(rename = "WeapDP28_C")]
    WeapDP28,
    #[serde(rename = "WeapG18_C")]
    WeapG18,
    #[serde(rename = "WeapGroza_C")]
    WeapGroza,
    #[serde(rename = "WeapHK416_C")]
    WeapHK416,
    #[serde(rename = "WeapKar98k_C")]
    WeapKar98k,
    #[serde(rename = "WeapM16A4_C")]
    WeapM16A4,
    #[serde(rename = "WeapM1911_C")]
    WeapM1911,
    #[serde(rename = "WeapM249_C")]
    WeapM249,
    #[serde(rename = "WeapM24_C")]
    WeapM24,
    #[serde(rename = "WeapM9_C")]
    WeapM9,
    #[serde(rename = "WeapMachete_C")]
    WeapMachete,
    #[serde(rename = "WeapMini14_C")]
    WeapMini14,
    #[serde(rename = "WeapMk14_C")]
    WeapMk14,
    #[serde(rename = "WeapNagantM1895_C")]
    WeapNagantM1895,
    #[serde(rename = "WeapPan_C")]
    WeapPan,
    #[serde(rename = "WeapRhino_C")]
    WeapRhino,
    #[serde(rename = "WeapSaiga12_C")]
    WeapSaiga12,
    #[serde(rename = "WeapSawnoff_C")]
    WeapSawnoff,
    #[serde(rename = "WeapSCAR-L_C")]
    WeapSCARL,
    #[serde(rename = "WeapSickle_C")]
    WeapSickle,
    #[serde(rename = "WeapSKS_C")]
    WeapSKS,
    #[serde(rename = "WeapThompson_C")]
    WeapThompson,
    #[serde(rename = "WeapUMP_C")]
    WeapUMP,
    #[serde(rename = "WeapUZI_C")]
    WeapUZI,
    #[serde(rename = "WeapVector_C")]
    WeapVector,
    #[serde(rename = "WeapVSS_C")]
    WeapVSS,
    #[serde(rename = "WeapWin94_C")]
    WeapWin94,
    #[serde(rename = "WeapWinchester_C")]
    WeapWinchester,
}

impl fmt::Display for DamageCauserName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            DamageCauserName::BattleRoyaleModeControllerDef
            | DamageCauserName::BattleRoyaleModeControllerDesert => "Bluezone",
            DamageCauserName::Pg117 => "PG-117",
            |
            DamageCauserName::Motorbike | DamageCauserName::MotorbikeDesert => "Motorcycle",
            |
            DamageCauserName::MotorbikeSideCar | DamageCauserName::MotorbikeSideCarDesert => {
                "Motorcycle (w/ Sidecar)"
            } |
            DamageCauserName::PickupTruckA1
            | DamageCauserName::PickupTruckA2
            | DamageCauserName::PickupTruckA3
            | DamageCauserName::PickupTruckA4
            | DamageCauserName::PickupTruckA5
            | DamageCauserName::PickupTruckB1
            | DamageCauserName::PickupTruckB2
            | DamageCauserName::PickupTruckB3
            | DamageCauserName::PickupTruckB4
            | DamageCauserName::PickupTruckB5 => "Pickup Truck",
            DamageCauserName::VanA1 | DamageCauserName::VanA2 | DamageCauserName::VanA3 => "Van",
            DamageCauserName::BuffDecreaseBreathInApnea => "Drowning",
            DamageCauserName::BuffFireDOT => "Burning",
            DamageCauserName::BuggyA1
            | DamageCauserName::BuggyA2
            | DamageCauserName::BuggyA3
            | DamageCauserName::BuggyA4
            | DamageCauserName::BuggyA5
            | DamageCauserName::BuggyA6 => "Buggy",
            DamageCauserName::DaciaA1
            | DamageCauserName::DaciaA2
            | DamageCauserName::DaciaA3
            | DamageCauserName::DaciaA4 => "Dacia",
            DamageCauserName::PlayerFemale => "Player",
            DamageCauserName::PlayerMale => "Player",
            DamageCauserName::ProjGrenade => "Grenade",
            DamageCauserName::ProjMolotov => "Molotov Cocktail",
            DamageCauserName::ProjMolotovDamageField
            | DamageCauserName::ProjMolotovDamageFieldInWall => "Molotov Cocktail Fire Field",
            DamageCauserName::RedZoneBomb => "Redzone",
            DamageCauserName::UazA1 | DamageCauserName::UazB1 | DamageCauserName::UazC1 => "UAZ",
            DamageCauserName::WeapAK47 => "AKM",
            DamageCauserName::WeapAUG => "AUG A3",
            DamageCauserName::WeapAWM => "AWM",
            DamageCauserName::WeapBerreta686 => "S686",
            DamageCauserName::WeapCowbar => "Crowbar",
            DamageCauserName::WeapCrossbow => "Crossbow",
            DamageCauserName::WeapDP28 => "DP-28",
            DamageCauserName::WeapG18 => "P18C",
            DamageCauserName::WeapGroza => "Groza",
            DamageCauserName::WeapHK416 => "M416",
            DamageCauserName::WeapKar98k => "Kar98k",
            DamageCauserName::WeapM16A4 => "M16A4",
            DamageCauserName::WeapM1911 => "P1911",
            DamageCauserName::WeapM249 => "M249",
            DamageCauserName::WeapM24 => "M24",
            DamageCauserName::WeapM9 => "P92",
            DamageCauserName::WeapMachete => "Machete",
            DamageCauserName::WeapMini14 => "Mini 14",
            DamageCauserName::WeapMk14 => "Mk14 EBR",
            DamageCauserName::WeapNagantM1895 => "R1895",
            DamageCauserName::WeapPan => "Pan",
            DamageCauserName::WeapRhino => "R45",
            DamageCauserName::WeapSaiga12 => "S12K",
            DamageCauserName::WeapSawnoff => "Sawed-off",
            DamageCauserName::WeapSCARL => "SCAR-L",
            DamageCauserName::WeapSickle => "Sickle",
            DamageCauserName::WeapSKS => "SKS",
            DamageCauserName::WeapThompson => "Tommy Gun",
            DamageCauserName::WeapUMP => "UMP9",
            DamageCauserName::WeapUZI => "Micro Uzi",
            DamageCauserName::WeapVector => "Vector",
            DamageCauserName::WeapVSS => "VSS",
            DamageCauserName::WeapWin94 => "Win94",
            DamageCauserName::WeapWinchester => "S1897",
        };
        f.write_str(msg)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DamageTypeCategory {
    #[serde(rename = "Damage_BlueZone")]
    BlueZone,
    #[serde(rename = "Damage_Drown")]
    Drown,
    #[serde(rename = "Damage_Explosion_Grenade")]
    ExplosionGrenade,
    #[serde(rename = "Damage_Explosion_RedZone")]
    ExplosionRedZone,
    #[serde(rename = "Damage_Explosion_Vehicle")]
    ExplosionVehicle,
    #[serde(rename = "Damage_Groggy")]
    Groggy,
    #[serde(rename = "Damage_Gun")]
    Gun,
    #[serde(rename = "Damage_Instant_Fall")]
    InstantFall,
    #[serde(rename = "Damage_Melee")]
    Melee,
    #[serde(rename = "Damage_Molotov")]
    Molotov,
    #[serde(rename = "Damage_VehicleCrashHit")]
    VehicleCrashHit,
    #[serde(rename = "Damage_VehicleHit")]
    VehicleHit,
}

impl fmt::Display for DamageTypeCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            DamageTypeCategory::BlueZone => "Bluezone Damage",
            DamageTypeCategory::Drown => "Drowning Damage",
            DamageTypeCategory::ExplosionGrenade => "Grenade Explosion Damage",
            DamageTypeCategory::ExplosionRedZone => "Redzone Explosion Damage",
            DamageTypeCategory::ExplosionVehicle => "Vehicle Explosion Damage",
            DamageTypeCategory::Groggy => "Bleed out damage",
            DamageTypeCategory::Gun => "Gun Damage",
            DamageTypeCategory::InstantFall => "Fall Damage",
            DamageTypeCategory::Melee => "Melee Damage",
            DamageTypeCategory::Molotov => "Molotov Damage",
            DamageTypeCategory::VehicleCrashHit => "Vehicle Crash Damage",
            DamageTypeCategory::VehicleHit => "Vehicle Damage",
        };
        f.write_str(msg)
    }
}
