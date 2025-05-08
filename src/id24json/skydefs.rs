use std::fmt::{Display, Formatter};

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct Fire {
    updatetime: f32,
    palette: Vec<u8>
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct ForegroundTex {
    name: String,
    mid: u32,
    scrollx: f32,
    scrolly: f32,
    scalex: f32,
    scaley: f32,
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum SkyType {
    Standard = 0,
    Fire = 1,
    WithForeground = 2
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct Sky {
    #[serde(rename = "type")]
    pub sky_type: SkyType,
    pub name: String,
    pub mid: u32,
    pub scrollx: f32,
    pub scrolly: f32,
    pub scalex: f32,
    pub scaley: f32,
    pub fire: Option<Fire>,
    pub foregroundtex: Option<ForegroundTex>
}

impl Default for Sky {
    fn default() -> Self {
        Self {
            sky_type: SkyType::Standard,
            name: "SKY1".to_owned(),
            mid: 100,
            scrollx: 0.0,
            scrolly: 0.0,
            scalex: 1.0,
            scaley: 1.0,
            fire: None,
            foregroundtex: None
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct FlatMapping {
    pub flat: String,
    pub sky: String
}

impl Default for FlatMapping {
    fn default() -> Self {
        Self {
            flat: "F_SKY1".to_owned(),
            sky: "SKY1".to_owned()
        }
    }
}

impl Display for FlatMapping {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.flat.clone())
    }
}

impl Display for Sky {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.clone())
    }
}