use std::fmt::Formatter;

#[derive(serde::Serialize, serde::Deserialize, strum_macros::VariantArray, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Options {
    // options available in doom1.9
    comp_soul: Option<bool>,
    comp_finaldoomteleport: Option<bool>,
    // options avaiable in limitremoving
    comp_texwidthclamp: Option<TexWidthClamp>,
    comp_clipmasked: Option<ClipMasked>,
    // options available in boom2.02
    comp_thingfloorlight: Option<bool>,
    // options available in complevel9
    comp_musinfo: Option<bool>,
    // options available in mbf
    comp_moveblock: Option<bool>,
    weapon_recoil: Option<bool>,
    monsters_remember: Option<bool>,
    monster_infighting: Option<bool>,
    monster_backing: Option<bool>,
    monster_avoid_hazards: Option<bool>,
    monkeys: Option<bool>,
    monster_friction: Option<bool>,
    help_friends: Option<bool>,
    player_helpers: Option<u8>,
    friend_distance: Option<u8>,
    dog_jumping: Option<bool>,
    comp_telefrag: Option<bool>,
    comp_dropoff: Option<bool>,
    comp_vile: Option<bool>,
    comp_pain: Option<bool>,
    comp_skull: Option<bool>,
    comp_blazing: Option<bool>,
    comp_doorlight: Option<bool>,
    comp_model: Option<bool>,
    comp_god: Option<bool>,
    comp_falloff: Option<bool>,
    comp_floors: Option<bool>,
    comp_skymap: Option<bool>,
    comp_pursuit: Option<bool>,
    comp_doorstuck: Option<bool>,
    comp_staylift: Option<bool>,
    comp_zombie: Option<bool>,
    comp_stairs: Option<bool>,
    comp_infcheat: Option<bool>,
    comp_zerotags: Option<bool>,
    comp_respawn: Option<bool>,
    // options available in mbf21
    comp_ledgeblock: Option<bool>,
    comp_friendlyspawn: Option<bool>,
    comp_voodooscroller: Option<bool>,
    comp_reservedlineflag: Option<bool>
}

#[derive(Clone, PartialEq, Debug)]
pub enum ClipMasked {
    None,
    MultipatchOnly,
    All
}

#[derive(Clone, PartialEq, Debug)]
pub enum TexWidthClamp {
    All,
    SolidWallsOnly,
    None
}

impl Options {
    pub fn set_executable(&mut self, exe: Executable) {
        if exe < Executable::LimitRemoving {
            self.comp_texwidthclamp = None;
            self.comp_clipmasked = None;
        }
        if exe < Executable::Boom2_02 {
            self.comp_thingfloorlight = None;
        }
        if exe < Executable::CompLevel9 {
            self.comp_musinfo = None;
        }
        if exe < Executable::MBF {
            self.comp_moveblock = None;
            self.weapon_recoil = None;
            self.monsters_remember = None;
            self.monster_infighting = None;
            self.monster_backing = None;
            self.monster_avoid_hazards = None;
            self.monkeys = None;
            self.monster_friction = None;
            self.help_friends = None;
            self.player_helpers = None;
            self.friend_distance = None;
            self.dog_jumping = None;
            self.comp_telefrag = None;
            self.comp_dropoff = None;
            self.comp_vile = None;
            self.comp_pain = None;
            self.comp_skull = None;
            self.comp_blazing = None;
            self.comp_doorlight = None;
            self.comp_model = None;
            self.comp_god = None;
            self.comp_falloff = None;
            self.comp_floors = None;
            self.comp_skymap = None;
            self.comp_pursuit = None;
            self.comp_doorstuck = None;
            self.comp_staylift = None;
            self.comp_zombie = None;
            self.comp_stairs = None;
            self.comp_infcheat = None;
            self.comp_zerotags = None;
            self.comp_respawn = None;
        }
        if exe < Executable::MBF21 {
            self.comp_ledgeblock = None;
            self.comp_friendlyspawn = None;
            self.comp_voodooscroller = None;
            self.comp_reservedlineflag = None;
        }
    }
}

impl serde::Serialize for Options {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        todo!()
    }
}

impl<'a> serde::Deserialize<'a> for Options {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'a> {
        let s = String::deserialize(deserializer)?;
        let mut options = Options::default();
        
        for line in s.lines() {
            let mut parts = line.split_whitespace();
            if let (Some(opt), Some(value)) = (parts.next(), parts.next()) {
                let enabled = || match value {
                    "0" => false,
                    "1" => true,
                    _ => panic!("Invalid value for option {opt}")
                };
                todo!()
            }
        }
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::id24json::gameconf::TexWidthClamp::SolidWallsOnly;
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
    #[test]
    fn deserialize_single_option() {
        let json = r#"{
            "options": "comp_soul 0"
        }"#;
        let options: Options = serde_json::from_str(json).unwrap();
        assert_eq!(options.comp_soul, Some(false));
        assert_eq!(options.comp_texwidthclamp, None);
        assert_eq!(options.comp_clipmasked, None);
    }
    #[test]
    fn deserialize_multiple_options() {
        let json = r#"{
            "options": "comp_soul 0
                        comp_texwidthclamp 1"
        }"#;
        let options: Options = serde_json::from_str(json).unwrap();
        assert_eq!(options.comp_soul, Some(false));
        assert_eq!(options.comp_texwidthclamp, Some(SolidWallsOnly));
        assert_eq!(options.comp_clipmasked, None);
    }
    #[test]
    fn deserialize_options_single_line() {
        let json = r#"{
            "options": "comp_soul 0 comp_texwidthclamp 1"
        }"#;
        assert!(serde_json::from_str::<Options>(json).is_err());
    }
}