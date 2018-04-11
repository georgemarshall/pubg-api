use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum VehicleType {
    FloatingVehicle,
    Parachute,
    TransportAircraft,
    WheeledVehicle,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VehicleId {
    #[serde(rename = "AquaRail_A_01_C")]
    AquaRail,
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
    #[serde(rename = "DummyTransportAircraft_C")]
    DummyTransportAircraft,
    #[serde(rename = "BP_Motorbike_04_C")]
    Motorbike,
    #[serde(rename = "BP_Motorbike_04_Desert_C")]
    MotorbikeDesert,
    #[serde(rename = "BP_Motorbike_04_SideCar_C")]
    MotorbikeSideCar,
    #[serde(rename = "BP_Motorbike_04_SideCar_Desert_C")]
    MotorbikeSideCarDesert,
    #[serde(rename = "ParachutePlayer_C")]
    ParachutePlayer,
    #[serde(rename = "Boat_PG117_C")]
    Pg117,
    #[serde(rename = "PG117_A_01_C")]
    Pg117A1,
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
    #[serde(rename = "Uaz_A_01_C")]
    UazA1,
    #[serde(rename = "Uaz_B_01_C")]
    UazB2,
    #[serde(rename = "Uaz_C_01_C")]
    UazC3,
}

impl fmt::Display for VehicleId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            VehicleId::AquaRail => "Aquarail",
            VehicleId::BuggyA1
            | VehicleId::BuggyA2
            | VehicleId::BuggyA3
            | VehicleId::BuggyA4
            | VehicleId::BuggyA5
            | VehicleId::BuggyA6 => "Buggy",
            VehicleId::DaciaA1 | VehicleId::DaciaA2 | VehicleId::DaciaA3 | VehicleId::DaciaA4 => {
                "Dacia 1300"
            }
            VehicleId::DummyTransportAircraft => "C-130",
            VehicleId::Motorbike | VehicleId::MotorbikeDesert => "Motorcycle",
            VehicleId::MotorbikeSideCar | VehicleId::MotorbikeSideCarDesert => {
                "Motorcycle (w/ Sidecar)"
            }
            VehicleId::ParachutePlayer => "Parachute",
            VehicleId::Pg117 | VehicleId::Pg117A1 => "PG-117",
            VehicleId::PickupTruckA1
            | VehicleId::PickupTruckA2
            | VehicleId::PickupTruckA3
            | VehicleId::PickupTruckA4
            | VehicleId::PickupTruckA5
            | VehicleId::PickupTruckB1
            | VehicleId::PickupTruckB2
            | VehicleId::PickupTruckB3
            | VehicleId::PickupTruckB4
            | VehicleId::PickupTruckB5 => "Pickup Truck",
            VehicleId::VanA1 | VehicleId::VanA2 | VehicleId::VanA3 => "Van",
            VehicleId::UazA1 | VehicleId::UazB2 | VehicleId::UazC3 => "UAZ",
        };
        f.write_str(msg)
    }
}
