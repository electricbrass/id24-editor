#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct Entry {
    primarylump: String,
    secondarylump: String,
    duration: f32,
    #[serde(rename = "type")]
    demo_type: DemoType,
    outrowwipe: OutRowWipe
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum DemoType {
    ArtScreen = 0,
    DemoLump = 1
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum OutRowWipe {
    Immediate = 0,
    ScreenMelt = 1
}