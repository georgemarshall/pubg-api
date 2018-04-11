use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum Category {
    Ammunition,
    Attachment,
    Equipment,
    Use,
    Weapon,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubCategory {
    Backpack,
    Boost,
    Fuel,
    Handgun,
    Headgear,
    Heal,
    Jacket,
    Main,
    Melee,
    None,
    Throwable,
    Vest,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ItemId {
    #[serde(rename = "Item_Ammo_12Guage_C")]
    Ammo12Guage,
    #[serde(rename = "Item_Ammo_300Magnum_C")]
    Ammo300Magnum,
    #[serde(rename = "Item_Ammo_45ACP_C")]
    Ammo45ACP,
    #[serde(rename = "Item_Ammo_556mm_C")]
    Ammo556mm,
    #[serde(rename = "Item_Ammo_762mm_C")]
    Ammo762mm,
    #[serde(rename = "Item_Ammo_9mm_C")]
    Ammo9mm,
    #[serde(rename = "Item_Ammo_Bolt_C")]
    AmmoBolt,
    #[serde(rename = "Item_Armor_C_01_Lv3_C")]
    ArmorLv3,
    #[serde(rename = "Item_Armor_D_01_Lv2_C")]
    ArmorLv2,
    #[serde(rename = "Item_Armor_E_01_Lv1_C")]
    ArmorLv1,
    #[serde(rename = "Item_Attach_Weapon_Lower_AngledForeGrip_C")]
    AttachWeaponLowerAngledForeGrip,
    #[serde(rename = "Item_Attach_Weapon_Lower_Foregrip_C")]
    AttachWeaponLowerForegrip,
    #[serde(rename = "Item_Attach_Weapon_Lower_QuickDraw_Large_Crossbow_C")]
    AttachWeaponLowerQuickDrawLargeCrossbow,
    #[serde(rename = "Item_Attach_Weapon_Magazine_Extended_Large_C")]
    AttachWeaponMagazineExtendedLarge,
    #[serde(rename = "Item_Attach_Weapon_Magazine_Extended_Medium_C")]
    AttachWeaponMagazineExtendedMedium,
    #[serde(rename = "Item_Attach_Weapon_Magazine_ExtendedQuickDraw_Large_C")]
    AttachWeaponMagazineExtendedQuickDrawLarge,
    #[serde(rename = "Item_Attach_Weapon_Magazine_ExtendedQuickDraw_Medium_C")]
    AttachWeaponMagazineExtendedQuickDrawMedium,
    #[serde(rename = "Item_Attach_Weapon_Magazine_ExtendedQuickDraw_Small_C")]
    AttachWeaponMagazineExtendedQuickDrawSmall,
    #[serde(rename = "Item_Attach_Weapon_Magazine_ExtendedQuickDraw_SniperRifle_C")]
    AttachWeaponMagazineExtendedQuickDrawSniperRifle,
    #[serde(rename = "Item_Attach_Weapon_Magazine_Extended_Small_C")]
    AttachWeaponMagazineExtendedSmall,
    #[serde(rename = "Item_Attach_Weapon_Magazine_Extended_SniperRifle_C")]
    AttachWeaponMagazineExtendedSniperRifle,
    #[serde(rename = "Item_Attach_Weapon_Magazine_QuickDraw_Large_C")]
    AttachWeaponMagazineQuickDrawLarge,
    #[serde(rename = "Item_Attach_Weapon_Magazine_QuickDraw_Medium_C")]
    AttachWeaponMagazineQuickDrawMedium,
    #[serde(rename = "Item_Attach_Weapon_Magazine_QuickDraw_Small_C")]
    AttachWeaponMagazineQuickDrawSmall,
    #[serde(rename = "Item_Attach_Weapon_Magazine_QuickDraw_SniperRifle_C")]
    AttachWeaponMagazineQuickDrawSniperRifle,
    #[serde(rename = "Item_Attach_Weapon_Muzzle_Choke_C")]
    AttachWeaponMuzzleChoke,
    #[serde(rename = "Item_Attach_Weapon_Muzzle_Compensator_Large_C")]
    AttachWeaponMuzzleCompensatorLarge,
    #[serde(rename = "Item_Attach_Weapon_Muzzle_Compensator_Medium_C")]
    AttachWeaponMuzzleCompensatorMedium,
    #[serde(rename = "Item_Attach_Weapon_Muzzle_Compensator_SniperRifle_C")]
    AttachWeaponMuzzleCompensatorSniperRifle,
    #[serde(rename = "Item_Attach_Weapon_Muzzle_FlashHider_Large_C")]
    AttachWeaponMuzzleFlashHiderLarge,
    #[serde(rename = "Item_Attach_Weapon_Muzzle_FlashHider_Medium_C")]
    AttachWeaponMuzzleFlashHiderMedium,
    #[serde(rename = "Item_Attach_Weapon_Muzzle_FlashHider_SniperRifle_C")]
    AttachWeaponMuzzleFlashHiderSniperRifle,
    #[serde(rename = "Item_Attach_Weapon_Muzzle_Suppressor_Large_C")]
    AttachWeaponMuzzleSuppressorLarge,
    #[serde(rename = "Item_Attach_Weapon_Muzzle_Suppressor_Medium_C")]
    AttachWeaponMuzzleSuppressorMedium,
    #[serde(rename = "Item_Attach_Weapon_Muzzle_Suppressor_Small_C")]
    AttachWeaponMuzzleSuppressorSmall,
    #[serde(rename = "Item_Attach_Weapon_Muzzle_Suppressor_SniperRifle_C")]
    AttachWeaponMuzzleSuppressorSniperRifle,
    #[serde(rename = "Item_Attach_Weapon_Stock_AR_Composite_C")]
    AttachWeaponStockARComposite,
    #[serde(rename = "Item_Attach_Weapon_Stock_Shotgun_BulletLoops_C")]
    AttachWeaponStockShotgunBulletLoops,
    #[serde(rename = "Item_Attach_Weapon_Stock_SniperRifle_BulletLoops_C")]
    AttachWeaponStockSniperRifleBulletLoops,
    #[serde(rename = "Item_Attach_Weapon_Stock_SniperRifle_CheekPad_C")]
    AttachWeaponStockSniperRifleCheekPad,
    #[serde(rename = "Item_Attach_Weapon_Stock_UZI_C")]
    AttachWeaponStockUZI,
    #[serde(rename = "Item_Attach_Weapon_Upper_ACOG_01_C")]
    AttachWeaponUpperACOG,
    #[serde(rename = "Item_Attach_Weapon_Upper_Aimpoint_C")]
    AttachWeaponUpperAimpoint,
    #[serde(rename = "Item_Attach_Weapon_Upper_CQBSS_C")]
    AttachWeaponUpperCQBSS,
    #[serde(rename = "Item_Attach_Weapon_Upper_DotSight_01_C")]
    AttachWeaponUpperDotSight,
    #[serde(rename = "Item_Attach_Weapon_Upper_Holosight_C")]
    AttachWeaponUpperHolosight,
    #[serde(rename = "Item_Attach_Weapon_Upper_PM2_01_C")]
    AttachWeaponUpperPM2,
    #[serde(rename = "Item_Back_B_01_StartParachutePack_C")]
    BackStartParachutePack,
    #[serde(rename = "Item_Back_C_01_Lv3_C")]
    Back1Lv3,
    #[serde(rename = "Item_Back_C_02_Lv3_C")]
    Back2Lv3,
    #[serde(rename = "Item_Back_E_01_Lv1_C")]
    Back1Lv1,
    #[serde(rename = "Item_Back_E_02_Lv1_C")]
    Back2Lv1,
    #[serde(rename = "Item_Back_F_01_Lv2_C")]
    Back1Lv2,
    #[serde(rename = "Item_Back_F_02_Lv2_C")]
    Back2Lv2,
    #[serde(rename = "Item_Boost_AdrenalineSyringe_C")]
    BoostAdrenalineSyringe,
    #[serde(rename = "Item_Boost_EnergyDrink_C")]
    BoostEnergyDrink,
    #[serde(rename = "Item_Boost_PainKiller_C")]
    BoostPainKiller,
    #[serde(rename = "Item_Ghillie_01_C")]
    Ghillie1,
    #[serde(rename = "Item_Ghillie_02_C")]
    Ghillie2,
    #[serde(rename = "Item_Head_E_01_Lv1_C")]
    Head1Lv1,
    #[serde(rename = "Item_Head_E_02_Lv1_C")]
    Head2Lv1,
    #[serde(rename = "Item_Head_F_01_Lv2_C")]
    Head1Lv2,
    #[serde(rename = "Item_Head_F_02_Lv2_C")]
    Head2Lv2,
    #[serde(rename = "Item_Head_G_01_Lv3_C")]
    Head1Lv3,
    #[serde(rename = "Item_Heal_Bandage_C")]
    HealBandage,
    #[serde(rename = "Item_Heal_FirstAid_C")]
    HealFirstAid,
    #[serde(rename = "Item_Heal_MedKit_C")]
    HealMedKit,
    #[serde(rename = "Item_JerryCan_C")]
    JerryCan,
    #[serde(rename = "Item_Weapon_AK47_C")]
    WeaponAK47,
    #[serde(rename = "Item_Weapon_AUG_C")]
    WeaponAUG,
    #[serde(rename = "Item_Weapon_AWM_C")]
    WeaponAWM,
    #[serde(rename = "Item_Weapon_Berreta686_C")]
    WeaponBerreta686,
    #[serde(rename = "Item_Weapon_Cowbar_C")]
    WeaponCowbar,
    #[serde(rename = "Item_Weapon_Crossbow_C")]
    WeaponCrossbow,
    #[serde(rename = "Item_Weapon_DP28_C")]
    WeaponDP28,
    #[serde(rename = "Item_Weapon_FlashBang_C")]
    WeaponFlashBang,
    #[serde(rename = "Item_Weapon_G18_C")]
    WeaponG18,
    #[serde(rename = "Item_Weapon_Grenade_C")]
    WeaponGrenade,
    #[serde(rename = "Item_Weapon_Groza_C")]
    WeaponGroza,
    #[serde(rename = "Item_Weapon_HK416_C")]
    WeaponHK416,
    #[serde(rename = "Item_Weapon_Kar98k_C")]
    WeaponKar98k,
    #[serde(rename = "Item_Weapon_M16A4_C")]
    WeaponM16A4,
    #[serde(rename = "Item_Weapon_M1911_C")]
    WeaponM1911,
    #[serde(rename = "Item_Weapon_M249_C")]
    WeaponM249,
    #[serde(rename = "Item_Weapon_M24_C")]
    WeaponM24,
    #[serde(rename = "Item_Weapon_M9_C")]
    WeaponM9,
    #[serde(rename = "Item_Weapon_Machete_C")]
    WeaponMachete,
    #[serde(rename = "Item_Weapon_Mini14_C")]
    WeaponMini14,
    #[serde(rename = "Item_Weapon_Mk14_C")]
    WeaponMk14,
    #[serde(rename = "Item_Weapon_Molotov_C")]
    WeaponMolotov,
    #[serde(rename = "Item_Weapon_NagantM1895_C")]
    WeaponNagantM1895,
    #[serde(rename = "Item_Weapon_Pan_C")]
    WeaponPan,
    #[serde(rename = "Item_Weapon_Rhino_C")]
    WeaponRhino,
    #[serde(rename = "Item_Weapon_Saiga12_C")]
    WeaponSaiga12,
    #[serde(rename = "Item_Weapon_Sawnoff_C")]
    WeaponSawnoff,
    #[serde(rename = "Item_Weapon_SCAR-L_C")]
    WeaponSCARL,
    #[serde(rename = "Item_Weapon_Sickle_C")]
    WeaponSickle,
    #[serde(rename = "Item_Weapon_SKS_C")]
    WeaponSKS,
    #[serde(rename = "Item_Weapon_SmokeBomb_C")]
    WeaponSmokeBomb,
    #[serde(rename = "Item_Weapon_Thompson_C")]
    WeaponThompson,
    #[serde(rename = "Item_Weapon_UMP_C")]
    WeaponUMP,
    #[serde(rename = "Item_Weapon_UZI_C")]
    WeaponUZI,
    #[serde(rename = "Item_Weapon_Vector_C")]
    WeaponVector,
    #[serde(rename = "Item_Weapon_VSS_C")]
    WeaponVSS,
    #[serde(rename = "Item_Weapon_Win1894_C")]
    WeaponWin1894,
    #[serde(rename = "Item_Weapon_Winchester_C")]
    WeaponWinchester,
}

impl fmt::Display for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            ItemId::Ammo12Guage => "12 Gauge Ammo",
            ItemId::Ammo300Magnum => "300 Magnum Ammo",
            ItemId::Ammo45ACP => ".45 ACP Ammo",
            ItemId::Ammo556mm => "5.56mm Ammo",
            ItemId::Ammo762mm => "7.62mm Ammo",
            ItemId::Ammo9mm => "9mm Ammo",
            ItemId::AmmoBolt => "Crossbow Bolt",
            ItemId::ArmorLv3 => "Military Vest (Level 3)",
            ItemId::ArmorLv2 => "Police Vest (Level 2)",
            ItemId::ArmorLv1 => "Police Vest (Level 1)",
            ItemId::AttachWeaponLowerAngledForeGrip => "Angled Foregrip",
            ItemId::AttachWeaponLowerForegrip => "Vertical Foregrip",
            ItemId::AttachWeaponLowerQuickDrawLargeCrossbow => "QuickDraw Crossbow Quiver",
            ItemId::AttachWeaponMagazineExtendedLarge => "Large Extended Mag",
            ItemId::AttachWeaponMagazineExtendedMedium => "Medium Extended Mag",
            ItemId::AttachWeaponMagazineExtendedQuickDrawLarge => "Large Extended QuickDraw Mag",
            ItemId::AttachWeaponMagazineExtendedQuickDrawMedium => "Medium Extended QuickDraw Mag",
            ItemId::AttachWeaponMagazineExtendedQuickDrawSmall => "Small Extended QuickDraw Mag",
            ItemId::AttachWeaponMagazineExtendedQuickDrawSniperRifle => {
                "Sniper Rifle Extended QuickDraw Mag"
            }
            ItemId::AttachWeaponMagazineExtendedSmall => "Small Extended Mag",
            ItemId::AttachWeaponMagazineExtendedSniperRifle => "Sniper Rifle Extended Mag",
            ItemId::AttachWeaponMagazineQuickDrawLarge => "Large QuickDraw Mag",
            ItemId::AttachWeaponMagazineQuickDrawMedium => "Medium Quickdraw Mag",
            ItemId::AttachWeaponMagazineQuickDrawSmall => "Small Quickdraw Mag",
            ItemId::AttachWeaponMagazineQuickDrawSniperRifle => "Sniper Rifle Quickdraw Mag",
            ItemId::AttachWeaponMuzzleChoke => "Choke",
            ItemId::AttachWeaponMuzzleCompensatorLarge => "Large Compensator",
            ItemId::AttachWeaponMuzzleCompensatorMedium => "Medium Compensator",
            ItemId::AttachWeaponMuzzleCompensatorSniperRifle => "Sniper Rifle Compensator",
            ItemId::AttachWeaponMuzzleFlashHiderLarge => "Large Flash Hider",
            ItemId::AttachWeaponMuzzleFlashHiderMedium => "Medium Flash Hider",
            ItemId::AttachWeaponMuzzleFlashHiderSniperRifle => "Sniper Rifle Flash Hider",
            ItemId::AttachWeaponMuzzleSuppressorLarge => "Large Supressor",
            ItemId::AttachWeaponMuzzleSuppressorMedium => "Medium Supressor",
            ItemId::AttachWeaponMuzzleSuppressorSmall => "Small Supressor",
            ItemId::AttachWeaponMuzzleSuppressorSniperRifle => "Sniper Rifle Supressor",
            ItemId::AttachWeaponStockARComposite => "Tactical Stock",
            ItemId::AttachWeaponStockShotgunBulletLoops => "Shotgun Bullet Loops",
            ItemId::AttachWeaponStockSniperRifleBulletLoops => "Sniper Rifle Bullet Loops",
            ItemId::AttachWeaponStockSniperRifleCheekPad => "Sniper Rifle Cheek Pad",
            ItemId::AttachWeaponStockUZI => "Uzi Stock",
            ItemId::AttachWeaponUpperACOG => "4x ACOG Scope",
            ItemId::AttachWeaponUpperAimpoint => "2x Aimpoint Scope",
            ItemId::AttachWeaponUpperCQBSS => "8x CQBSS Scope",
            ItemId::AttachWeaponUpperDotSight => "Red Dot Sight",
            ItemId::AttachWeaponUpperHolosight => "Holographic Sight",
            ItemId::AttachWeaponUpperPM2 => "15x PM II Scope",
            ItemId::BackStartParachutePack => "Parachute",
            ItemId::Back1Lv3 | ItemId::Back2Lv3 => "Backpack (Level 3)",
            ItemId::Back1Lv2 | ItemId::Back2Lv2 => "Backpack (Level 2)",
            ItemId::Back1Lv1 | ItemId::Back2Lv1 => "Backpack (Level 1)",
            ItemId::BoostAdrenalineSyringe => "Adrenaline Syringe",
            ItemId::BoostEnergyDrink => "Energy Drink",
            ItemId::BoostPainKiller => "Painkiller",
            ItemId::Ghillie1 | ItemId::Ghillie2 => "Ghillie Suit",
            ItemId::Head1Lv1 | ItemId::Head2Lv1 => "Motorcycle Helmet (Level 1)",
            ItemId::Head1Lv2 | ItemId::Head2Lv2 => "Military Helmet (Level 2)",
            ItemId::Head1Lv3 => "Spetsnaz Helmet (Level 3)",
            ItemId::HealBandage => "Bandage",
            ItemId::HealFirstAid => "First Aid Kit",
            ItemId::HealMedKit => "Med kit",
            ItemId::JerryCan => "Gas Can",
            ItemId::WeaponAK47 => "AKM",
            ItemId::WeaponAUG => "AUG A3",
            ItemId::WeaponAWM => "AWM",
            ItemId::WeaponBerreta686 => "S686",
            ItemId::WeaponCowbar => "Crowbar",
            ItemId::WeaponCrossbow => "Crossbow",
            ItemId::WeaponDP28 => "DP-28",
            ItemId::WeaponFlashBang => "Flashbang",
            ItemId::WeaponG18 => "P18C",
            ItemId::WeaponGrenade => "Frag Grenade",
            ItemId::WeaponGroza => "Groza",
            ItemId::WeaponHK416 => "M416",
            ItemId::WeaponKar98k => "Kar98k",
            ItemId::WeaponM16A4 => "M16A4",
            ItemId::WeaponM1911 => "P1911",
            ItemId::WeaponM249 => "M249",
            ItemId::WeaponM24 => "M24",
            ItemId::WeaponM9 => "P92",
            ItemId::WeaponMachete => "Machete",
            ItemId::WeaponMini14 => "Mini 14",
            ItemId::WeaponMk14 => "Mk14 EBR",
            ItemId::WeaponMolotov => "Molotov Cocktail",
            ItemId::WeaponNagantM1895 => "R1895",
            ItemId::WeaponPan => "Pan",
            ItemId::WeaponRhino => "R45",
            ItemId::WeaponSaiga12 => "S12K",
            ItemId::WeaponSawnoff => "Sawed-off",
            ItemId::WeaponSCARL => "SCAR-L",
            ItemId::WeaponSickle => "Sickle",
            ItemId::WeaponSKS => "SKS",
            ItemId::WeaponSmokeBomb => "Smoke Genade",
            ItemId::WeaponThompson => "Tommy Gun",
            ItemId::WeaponUMP => "UMP9",
            ItemId::WeaponUZI => "Micro Uzi",
            ItemId::WeaponVector => "Vector",
            ItemId::WeaponVSS => "VSS",
            ItemId::WeaponWin1894 => "Win94",
            ItemId::WeaponWinchester => "S1897",
        };
        f.write_str(msg)
    }
}
