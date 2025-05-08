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
    None = 0,
    // TODO: name the conditions, odamex has names for them in its code that can be taken
    idk1 = 1,
    idk2 = 2,
    idk3 = 3,
    idk4 = 4,
    idk5 = 5,
    idk6 = 6,
    idk7 = 7,
}