pub mod skydefs;

use skydefs::{Sky, FlatMapping};

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
#[serde(tag = "type", content = "data", rename_all = "lowercase")]
pub enum ID24JsonData {
    GAMECONF,
    DEMOLOOP,
    SBARDEF,
    SKYDEFS {
        skies: Option<Vec<Sky>>,
        flatmapping: Option<Vec<FlatMapping>>
    },
    TRAKINFO,
    Interlevel,
    Finale
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ID24JsonMetaData {}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ID24Json {
    version: ID24JsonVersion,
    metadata: ID24JsonMetaData,
    #[serde(flatten)]
    pub data: ID24JsonData,
}

#[derive(PartialEq, Debug)]
struct ID24JsonVersion {
    major: u8,
    minor: u8,
    revision: u8,
}

impl serde::Serialize for ID24JsonVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(format!("{}.{}.{}", self.major, self.minor, self.revision).as_ref())
    }
}

impl<'a> serde::Deserialize<'a> for ID24JsonVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'a> {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(serde::de::Error::custom("Expected format 'major.minor.revision'"));
        }

        let major = parts[0].parse::<u8>()
            .map_err(|_| serde::de::Error::custom("Invalid major version"))?;
        let minor = parts[1].parse::<u8>()
            .map_err(|_| serde::de::Error::custom("Invalid minor version"))?;
        let revision = parts[2].parse::<u8>()
            .map_err(|_| serde::de::Error::custom("Invalid revision version"))?;

        Ok(ID24JsonVersion { major, minor, revision })
    }
}

impl Default for ID24Json {
    fn default() -> Self {
        Self {
            version: ID24JsonVersion { major: 1, minor: 0, revision: 0 },
            metadata: ID24JsonMetaData {},
            data: ID24JsonData::SKYDEFS {
                skies: None,
                flatmapping: None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::id24json::skydefs::SkyType;
    use super::*;
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
    fn fail_on_missing_root_fields() {
        let json = r#"{
            "type": "skydefs",
            "version": "1.0.0",
            "metadata": { },
            "data": null
        }"#;
        assert!(serde_json::from_str::<ID24Json>(json).is_err());
        let json = r#"{
            "type": "skydefs",
            "version": "1.0.0",
            "metadata": { },
        }"#;
        assert!(serde_json::from_str::<ID24Json>(json).is_err());
        let json = r#"{
            "type": "skydefs",
            "metadata": { },
            "data":
            {
                "skies": null,
                "flatmapping": null
            }
        }"#;
        assert!(serde_json::from_str::<ID24Json>(json).is_err());
        let json = r#"{
            "type": "skydefs",
            "version": "1.0.0",
            "data":
            {
                "skies": null,
                "flatmapping": null
            }
        }"#;
        assert!(serde_json::from_str::<ID24Json>(json).is_err());
    }
    #[test]
    fn fail_on_invalid_version() {
        let json = r#"{
            "type": "skydefs",
            "version": "1.0",
            "metadata": { }
            "data":
            {
                "skies": null,
                "flatmapping": null
            }
        }"#;
        assert!(serde_json::from_str::<ID24Json>(json).is_err());
        let json = r#"{
            "type": "skydefs",
            "version": "1.0.0.0",
            "metadata": { }
            "data":
            {
                "skies": null,
                "flatmapping": null
            }
        }"#;
        assert!(serde_json::from_str::<ID24Json>(json).is_err());
        let json = r#"{
            "type": "skydefs",
            "version": "one",
            "metadata": { }
            "data":
            {
                "skies": null,
                "flatmapping": null
            }
        }"#;
        assert!(serde_json::from_str::<ID24Json>(json).is_err());
        let json = r#"{
            "type": "skydefs",
            "version": 1.0.0,
            "metadata": { }
            "data":
            {
                "skies": null,
                "flatmapping": null
            }
        }"#;
        assert!(serde_json::from_str::<ID24Json>(json).is_err());
    }
}