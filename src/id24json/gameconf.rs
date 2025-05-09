use std::fmt::Formatter;

#[derive(serde::Serialize, serde::Deserialize, strum_macros::VariantArray, Clone, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Executable {
    #[serde(rename = "doom1.9")]
    Doom1_9,
    LimitRemoving,
    Bugfixed,
    #[serde(rename = "boom2.02")]
    Boom2_02,
    CompLevel9,
    MBF,
    MBF21,
    MBF21EX,
    ID24
}

impl std::fmt::Display for Executable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Executable::Doom1_9       => "Vanilla",
            Executable::LimitRemoving => "Limit Removing",
            Executable::Bugfixed      => "Bugfixed",
            Executable::Boom2_02      => "Boom 2.02",
            Executable::CompLevel9    => "Boom (CL9)",
            Executable::MBF           => "MBF",
            Executable::MBF21         => "MBF21",
            Executable::MBF21EX       => "MBF21 + DSDHacked",
            Executable::ID24          => "ID24"
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, strum_macros::VariantArray, Clone, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Registered,
    Retail,
    Commercial
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Mode::Registered => "Registered",
            Mode::Retail     => "Retail",
            Mode::Commercial => "Commercial",
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::*;
    #[test]
    fn read_gameconf() {
        let json = r#"{
            "type": "gameconf",
            "version": "1.0.0",
            "metadata": { },
            "data":
            {
                "title": "A Totally Real WAD",
                "author": "electricbrass",
                "description": null,
                "version": "1.0",
                "iwad": "doom2.wad",
                "pwadfiles": null,
                "dehfiles": null,
                "executable": "doom1.9",
                "mode": "commercial",
                "options": null
            }
        }"#;
        let data: ID24Json = serde_json::from_str(json).unwrap();
        assert_eq!(data.version, ID24JsonVersion { major: 1, minor: 0, revision: 0 });
        assert_eq!(data.data, ID24JsonData::GAMECONF {
            title: Some("A Totally Real WAD".to_owned()),
            author: Some("electricbrass".to_owned()),
            description: None,
            version: Some("1.0".to_owned()),
            iwad: Some("doom2.wad".to_owned()),
            pwadfiles: None,
            dehfiles: None,
            executable: Some(Executable::Doom1_9),
            mode: Some(Mode::Commercial),
            options: None,
            playertranslations: None,
            wadtranslation: None
        });
    }
}