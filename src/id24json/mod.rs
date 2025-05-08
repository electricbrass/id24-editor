pub mod skydefs;

use skydefs::{Sky, FlatMapping, SkyType};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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

#[derive(Debug)]
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