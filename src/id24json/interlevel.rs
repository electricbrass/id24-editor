#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct Layer {
    anims: Vec<Anim>,
    conditions: Option<Vec<Condition>>
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct Anim {
    x: u16,
    y: u16,
    frames: Vec<Frame>,
    conditions: Option<Vec<Condition>>
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct Frame {
    image: String,
    #[serde(rename = "type")]
    frame_type: u16, // TODO: change this to an enum maybe? ask why it's a bitfield if only one can ever be selected at a time
    duration: f32,
    maxduration: f32,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct Condition {
    condition: ConditionType,
    param: u8
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
#[repr(u8)]
enum ConditionType {
    None             = 0,
    CurrMapGreater   = 1, // Current map number is greater than the param value
    CurrMapEqual     = 2, // Current map number is equal to the param value
    MapVisited       = 3, // The map number corresponding to the param value has been visited
    CurrMapNotSecret = 4, // The current map is not a secret map
    AnySecretVisited = 5, // Any secret map has been visited
    OnFinishedScreen = 6, // The current screen is the "finished" screen
    OnEnteringScreen = 7, // The current screen is the “entering” screen
}