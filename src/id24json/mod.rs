pub mod skydefs;
mod gameconf;
mod demoloop;
mod interlevel;
mod finale;
mod sbardef;

use skydefs::{Sky, FlatMapping};

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
#[serde(tag = "type", content = "data", rename_all = "lowercase")]
pub enum ID24JsonData {
    GAMECONF {
        title: Option<String>,
        author: Option<String>,
        description: Option<String>,
        version: Option<String>,
        iwad: Option<String>,
        // TODO: ask for clarification on spec, notes below
        pwadfiles: Option<Vec<String>>, // spec says its called pwads, official GAMECONFS use pwadfiles
        dehfiles: Option<Vec<String>>, // not mentioned at all in spec but present in official GAMECONFS
        playertranslations: Option<Vec<String>>, // spec says it can be null, does *not* say it can be undefined. it is undefined in official GAMECONFS
        wadtranslation: Option<Vec<String>>, // same as above
        executable: Option<gameconf::Executable>,
        mode: Option<gameconf::Mode>,
        options: Option<String> // TODO: make an actual type for the options
    },
    DEMOLOOP {
        entries: Vec<demoloop::Entry>
    },
    #[serde(rename = "statusbar")]
    SBARDEF {
        numberfonts: Vec<sbardef::NumberFont>,
        statusbars: Vec<sbardef::StatusBar>
    },
    SKYDEFS {
        skies: Option<Vec<Sky>>,
        flatmapping: Option<Vec<FlatMapping>>
    },
    TRAKINFO, // TODO: split this out for now, but i hope that formalized TRAKINFO ends up using the same root
    Interlevel {
        backgroundimage: String,
        music: String,
        layers: Option<Vec<interlevel::Layer>>
    },
    Finale {
        #[serde(rename = "type")]
        finale_type: finale::Type,
        music: String,
        background: String,
        donextmap: bool,
        bunny: finale::Bunny,
        castrollcall: finale::CastRollCall
    }
}

// TODO: verify the JSON is valid, serde handles most of this but theres a couple extra restrictions from ID24
// interlevel layers must be null or non-empty, not an empty array
// sky fire and foregroundtex must be null or non-null depending on type
// numberfonts and statusbars must not be empty
// numberfont stem length limits
// all conditions arrays must be non-empty or null
// all animation frame arrays must be non-empty
// ...

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ID24Json {
    version: ID24JsonVersion,
    metadata: Option<serde_json::Value>, // ID24 spec says this can't ever be null but LoR has null in its SBARDEF
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
            metadata: Some(serde_json::json!({ "application": "my cool editor :)" })),
            data: ID24JsonData::SKYDEFS {
                skies: None,
                flatmapping: None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
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