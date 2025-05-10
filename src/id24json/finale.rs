#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Type {
    ArtScreen = 0,
    BunnyScroller = 1,
    CastRollCall = 2
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct Bunny {
    stitchimage: String,
    // TODO: check if u32 is the right type to use here
    overlay: u32,
    overlaycount: u32,
    overlaysound: u32,
    overlayx: u32,
    overlayy: u32
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct CastRollCall {
    castmembers: Vec<CastMember>
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
struct CastMember {
    // TODO: find out what goes here, seems to not be in the spec
}