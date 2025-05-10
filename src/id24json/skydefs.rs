use std::fmt::{Display, Formatter};

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct Fire {
    pub updatetime: f32,
    pub palette: Vec<u8>
}

impl Default for Fire {
    fn default() -> Self {
        Self {
            updatetime: 0.05715, // 2 tics
            palette: Vec::new() // make this be the PSX palette maybe?
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct ForegroundTex {
    pub name: String,
    pub mid: u16,
    pub scrollx: f32,
    pub scrolly: f32,
    pub scalex: f32,
    pub scaley: f32,
}

impl Default for ForegroundTex {
    fn default() -> Self {
        Self {
            name: "SKY2".to_owned(),
            mid: 100,
            scrollx: 0.0,
            scrolly: 0.0,
            scalex: 1.0,
            scaley: 1.0,
        }
    }
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, strum_macros::VariantArray, PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum SkyType {
    Standard = 0,
    Fire = 1,
    WithForeground = 2
}

impl Display for SkyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SkyType::Standard        => "Standard",
            SkyType::Fire            => "Fire",
            SkyType::WithForeground  => "With Foreground",
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct Sky {
    #[serde(rename = "type")]
    pub sky_type: SkyType,
    pub name: String,
    pub mid: u16,
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

// TODO: decide if such a basic Display impl is the best way to go for displaying these in lists, might prefer to use a new trait and Display for something else
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

#[cfg(test)]
mod test {
    use super::*;
    use super::super::*;
    #[test]
    fn read_simple_skydefs() {
        let json = r#"{
            "type": "skydefs",
            "version": "1.0.0",
            "metadata": { },
            "data":
            {
                "skies":
                [
                    {
                        "type": 0,
                        "name": "SKY1",
                        "mid": 100,
                        "scrollx": 1,
                        "scrolly": 2,
                        "scalex": 3,
                        "scaley": 4,
		                "fire": null,
                        "foregroundtex": null
                    }
                ],
                "flatmapping":
                [
                    {
                        "flat" : "FLAT1",
                        "sky": "SKY1"
                    }
                ]
            }
        }"#;
        let data: ID24Json = serde_json::from_str(json).unwrap();
        assert_eq!(data.version, ID24JsonVersion { major: 1, minor: 0, revision: 0 });
        assert_eq!(data.data, ID24JsonData::SKYDEFS {
            skies: Some(vec![Sky {
                sky_type: SkyType::Standard,
                name: "SKY1".to_owned(),
                mid: 100,
                scrollx: 1.0,
                scrolly: 2.0,
                scalex: 3.0,
                scaley: 4.0,
                fire: None,
                foregroundtex: None
            }]),
            flatmapping: Some(vec![FlatMapping {
                flat: "FLAT1".to_owned(),
                sky: "SKY1".to_owned()
            }])
        });
        let json = r#"{
            "type": "skydefs",
            "version": "1.0.0",
            "metadata": { },
            "data":
            {
                "skies": null,
                "flatmapping": null
            }
        }"#;
        let data: ID24Json = serde_json::from_str(json).unwrap();
        assert_eq!(data.version, ID24JsonVersion { major: 1, minor: 0, revision: 0 });
        assert_eq!(data.data, ID24JsonData::SKYDEFS {
            skies: None,
            flatmapping: None
        });
    }
    #[test]
    // TODO: make this a real test and add more test files
    fn skydefs_test_2() {
        let data: ID24Json = serde_json::from_str(
            include_str!("test_files/skydefs_1.json")
        ).unwrap();
        assert_eq!(data.version, ID24JsonVersion { major: 1, minor: 0, revision: 0 });
    }
}