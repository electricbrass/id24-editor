/*
 * Copyright (C) 2025  Mia McMahill
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

use std::str::FromStr;
use std::fmt::{Display, Formatter};
use std::collections::BTreeMap;
use std::fmt::Write;
use strum::IntoEnumIterator;

#[derive(serde::Serialize, serde::Deserialize, strum_macros::VariantArray, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Executable {
    #[serde(rename = "doom1.9")]
    Doom1_9,
    LimitRemoving,
    #[serde(rename = "boom2.02")]
    Boom2_02,
    CompLevel9,
    MBF,
    MBF21,
    ID24
}

impl std::fmt::Display for Executable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Executable::Doom1_9       => "Vanilla",
            Executable::LimitRemoving => "Limit Removing",
            Executable::Boom2_02      => "Boom 2.02",
            Executable::CompLevel9    => "Boom (CL9)",
            Executable::MBF           => "MBF",
            Executable::MBF21         => "MBF21",
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

#[allow(non_camel_case_types)] // makes it easy to derive from and to string
#[derive(
    strum_macros::VariantArray,
    strum_macros::EnumIter,
    strum_macros::EnumString,
    strum_macros::Display,
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord
)]
pub enum CompOption {
    // options available in doom1.9
    comp_soul,
    comp_finaldoomteleport,
    // options available in limitremoving
    comp_texwidthclamp,
    comp_clipmasked,
    // options available in boom2.02
    comp_thingfloorlight,
    // options available in complevel9
    comp_musinfo,
    // options available in mbf
    comp_moveblock,
    weapon_recoil,
    monsters_remember,
    monster_infighting,
    monster_backing,
    monster_avoid_hazards,
    monkeys,
    monster_friction,
    help_friends,
    player_helpers,
    friend_distance,
    dog_jumping,
    comp_telefrag,
    comp_dropoff,
    comp_vile,
    comp_pain,
    comp_skull,
    comp_blazing,
    comp_doorlight,
    comp_model,
    comp_god,
    comp_falloff,
    comp_floors,
    comp_skymap,
    comp_pursuit,
    comp_doorstuck,
    comp_staylift,
    comp_zombie,
    comp_stairs,
    comp_infcheat,
    comp_zerotags,
    comp_respawn, // TODO: find out correct place for this, rnr says mbf21, doomwiki says mbf
    // options available in mbf21
    comp_ledgeblock,
    comp_friendlyspawn,
    comp_voodooscroller,
    comp_reservedlineflag,
    // TODO: find out complevels for these
    comp_666,
    comp_maskedanim,
    comp_ouchface,
    comp_maxhealth,
    comp_sound
}

impl CompOption {
    pub fn min_exe(self) -> Executable {
        match self {
            Self::comp_soul |
            Self::comp_finaldoomteleport => Executable::Doom1_9,
            Self::comp_texwidthclamp |
            Self::comp_clipmasked => Executable::LimitRemoving,
            Self::comp_thingfloorlight => Executable::Boom2_02,
            Self::comp_musinfo => Executable::CompLevel9,
            Self::comp_ledgeblock |
            Self::comp_friendlyspawn |
            Self::comp_voodooscroller |
            Self::comp_reservedlineflag => Executable::MBF21,
            // TODO: explicitly list these because this is likely to result in bugs when future options are added
            _ => Executable::MBF
        }
    }

    pub fn max_exe(self) -> Executable {
        match self {
            Self::comp_moveblock |
            Self::comp_666 |
            Self::comp_maskedanim |
            Self::comp_ouchface |
            Self::comp_maxhealth |
            Self::comp_sound => Executable::MBF,
            _ => Executable::ID24 // TODO: make this always the highest, or maybe None?
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn default_value(self, exe: Option<Executable>) -> Option<OptionValue> {
        match exe {
            #[allow(clippy::match_same_arms)]
            Some(Executable::Doom1_9) => match self {
                Self::comp_soul              => Some(OptionValue::Bool(false)),
                Self::comp_finaldoomteleport => Some(OptionValue::Bool(false)),
                _ => None
            },
            #[allow(clippy::match_same_arms)]
            Some(Executable::LimitRemoving) => match self {
                Self::comp_soul              => Some(OptionValue::Bool(false)),
                Self::comp_finaldoomteleport => Some(OptionValue::Bool(false)),

                Self::comp_texwidthclamp     => Some(OptionValue::TexWidthClamp(TexWidthClamp::All)),
                Self::comp_clipmasked        => Some(OptionValue::ClipMasked(ClipMasked::None)),
                _ => None
            },
            #[allow(clippy::match_same_arms)]
            Some(Executable::Boom2_02) => match self {
                Self::comp_soul              => Some(OptionValue::Bool(false)),
                Self::comp_finaldoomteleport => Some(OptionValue::Bool(false)),
                Self::comp_texwidthclamp     => Some(OptionValue::TexWidthClamp(TexWidthClamp::All)),
                Self::comp_clipmasked        => Some(OptionValue::ClipMasked(ClipMasked::MultipatchOnly)),

                Self::comp_thingfloorlight   => Some(OptionValue::Bool(false)),
                _ => None
            },
            #[allow(clippy::match_same_arms)]
            Some(Executable::CompLevel9) => match self {
                Self::comp_soul              => Some(OptionValue::Bool(false)),
                Self::comp_finaldoomteleport => Some(OptionValue::Bool(false)),
                Self::comp_texwidthclamp     => Some(OptionValue::TexWidthClamp(TexWidthClamp::SolidWallsOnly)),
                Self::comp_clipmasked        => Some(OptionValue::ClipMasked(ClipMasked::All)),
                Self::comp_thingfloorlight   => Some(OptionValue::Bool(true)),

                Self::comp_musinfo           => Some(OptionValue::Bool(true)),
                _ => None
            },
            #[allow(clippy::match_same_arms)]
            Some(Executable::MBF) => match self {
                Self::comp_soul              => Some(OptionValue::Bool(false)),
                Self::comp_finaldoomteleport => Some(OptionValue::Bool(false)),
                Self::comp_texwidthclamp     => Some(OptionValue::TexWidthClamp(TexWidthClamp::SolidWallsOnly)),
                Self::comp_clipmasked        => Some(OptionValue::ClipMasked(ClipMasked::All)),
                Self::comp_thingfloorlight   => Some(OptionValue::Bool(true)),
                Self::comp_musinfo           => Some(OptionValue::Bool(true)),

                Self::weapon_recoil          => Some(OptionValue::Bool(false)), // TODO: rnr has this as true for mbf, but doomwiki says false is the default
                Self::monsters_remember      => Some(OptionValue::Bool(true)),
                Self::monster_infighting     => Some(OptionValue::Bool(true)),
                Self::monster_backing        => Some(OptionValue::Bool(false)),
                Self::monster_avoid_hazards  => Some(OptionValue::Bool(true)),
                Self::monkeys                => Some(OptionValue::Bool(false)),
                Self::monster_friction       => Some(OptionValue::Bool(true)),
                Self::help_friends           => Some(OptionValue::Bool(true)),
                Self::player_helpers         => Some(OptionValue::Int(0)),
                Self::friend_distance        => Some(OptionValue::Int(128)),
                Self::dog_jumping            => Some(OptionValue::Bool(true)),
                Self::comp_telefrag          => Some(OptionValue::Bool(false)),
                Self::comp_dropoff           => Some(OptionValue::Bool(false)),
                Self::comp_vile              => Some(OptionValue::Bool(false)),
                Self::comp_pain              => Some(OptionValue::Bool(false)),
                Self::comp_skull             => Some(OptionValue::Bool(false)),
                Self::comp_blazing           => Some(OptionValue::Bool(false)),
                Self::comp_doorlight         => Some(OptionValue::Bool(false)),
                Self::comp_model             => Some(OptionValue::Bool(false)),
                Self::comp_god               => Some(OptionValue::Bool(false)),
                Self::comp_falloff           => Some(OptionValue::Bool(false)),
                Self::comp_floors            => Some(OptionValue::Bool(false)),
                Self::comp_skymap            => Some(OptionValue::Bool(false)),
                Self::comp_pursuit           => Some(OptionValue::Bool(true)),
                Self::comp_doorstuck         => Some(OptionValue::Bool(false)),
                Self::comp_staylift          => Some(OptionValue::Bool(false)),
                Self::comp_zombie            => Some(OptionValue::Bool(false)),
                Self::comp_stairs            => Some(OptionValue::Bool(true)),
                Self::comp_infcheat          => Some(OptionValue::Bool(false)),
                Self::comp_zerotags          => Some(OptionValue::Bool(false)),
                Self::comp_respawn           => Some(OptionValue::Bool(false)), // TODO: find out if this exists for this complevel and the proper default

                Self::comp_moveblock         => Some(OptionValue::Bool(true)),
                Self::comp_666               => Some(OptionValue::Bool(false)),
                Self::comp_maskedanim        => Some(OptionValue::Bool(true)),
                Self::comp_ouchface          => Some(OptionValue::Bool(true)),
                Self::comp_maxhealth         => Some(OptionValue::Bool(false)),
                Self::comp_sound             => Some(OptionValue::Bool(true)),
                _ => None
            },
            #[allow(clippy::match_same_arms)]
            // TODO: make sure i'm not wrong in assuming these are the same for mbf21 and id24
            Some(Executable::MBF21 | Executable::ID24) => match self {
                Self::comp_soul              => Some(OptionValue::Bool(false)),
                Self::comp_finaldoomteleport => Some(OptionValue::Bool(false)),
                Self::comp_texwidthclamp     => Some(OptionValue::TexWidthClamp(TexWidthClamp::SolidWallsOnly)),
                Self::comp_clipmasked        => Some(OptionValue::ClipMasked(ClipMasked::All)),
                Self::comp_thingfloorlight   => Some(OptionValue::Bool(true)),
                Self::comp_musinfo           => Some(OptionValue::Bool(true)),
                Self::weapon_recoil          => Some(OptionValue::Bool(false)), // TODO: rnr has this as true for mbf, but doomwiki says false is the default
                Self::monsters_remember      => Some(OptionValue::Bool(true)),
                Self::monster_infighting     => Some(OptionValue::Bool(true)),
                Self::monster_backing        => Some(OptionValue::Bool(false)),
                Self::monster_avoid_hazards  => Some(OptionValue::Bool(true)),
                Self::monkeys                => Some(OptionValue::Bool(false)),
                Self::monster_friction       => Some(OptionValue::Bool(true)),
                Self::help_friends           => Some(OptionValue::Bool(false)),
                Self::player_helpers         => Some(OptionValue::Int(0)),
                Self::friend_distance        => Some(OptionValue::Int(128)),
                Self::dog_jumping            => Some(OptionValue::Bool(true)),
                Self::comp_telefrag          => Some(OptionValue::Bool(false)),
                Self::comp_dropoff           => Some(OptionValue::Bool(false)),
                Self::comp_vile              => Some(OptionValue::Bool(false)),
                Self::comp_pain              => Some(OptionValue::Bool(false)),
                Self::comp_skull             => Some(OptionValue::Bool(false)),
                Self::comp_blazing           => Some(OptionValue::Bool(false)),
                Self::comp_doorlight         => Some(OptionValue::Bool(false)),
                Self::comp_model             => Some(OptionValue::Bool(false)),
                Self::comp_god               => Some(OptionValue::Bool(false)),
                Self::comp_falloff           => Some(OptionValue::Bool(false)),
                Self::comp_floors            => Some(OptionValue::Bool(false)),
                Self::comp_skymap            => Some(OptionValue::Bool(false)),
                Self::comp_pursuit           => Some(OptionValue::Bool(true)),
                Self::comp_doorstuck         => Some(OptionValue::Bool(false)),
                Self::comp_staylift          => Some(OptionValue::Bool(false)),
                Self::comp_zombie            => Some(OptionValue::Bool(true)),
                Self::comp_stairs            => Some(OptionValue::Bool(false)),
                Self::comp_infcheat          => Some(OptionValue::Bool(false)),
                Self::comp_zerotags          => Some(OptionValue::Bool(false)),
                Self::comp_respawn           => Some(OptionValue::Bool(false)),

                Self::comp_ledgeblock        => Some(OptionValue::Bool(true)),
                Self::comp_friendlyspawn     => Some(OptionValue::Bool(true)),
                Self::comp_voodooscroller    => Some(OptionValue::Bool(false)),
                Self::comp_reservedlineflag  => Some(OptionValue::Bool(true)),
                _ => None
            },
            None => None // TODO: options are allowed with null executable, but what should the defaults be??
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, strum_macros::FromRepr, strum_macros::VariantArray)]
#[repr(u16)]
pub enum ClipMasked {
    None           = 0,
    MultipatchOnly = 1,
    All            = 2
}

impl Display for ClipMasked {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ClipMasked::None           => "None",
            ClipMasked::MultipatchOnly => "Multi-patch only",
            ClipMasked::All            => "All"
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug, strum_macros::FromRepr, strum_macros::VariantArray)]
#[repr(u16)]
pub enum TexWidthClamp {
    All            = 0,
    SolidWallsOnly = 1,
    None           = 2
}

impl Display for TexWidthClamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TexWidthClamp::All            => "All",
            TexWidthClamp::SolidWallsOnly => "Solid walls only",
            TexWidthClamp::None           => "None"
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OptionValue {
    Bool(bool),
    Int(u16),
    ClipMasked(ClipMasked),
    TexWidthClamp(TexWidthClamp)
}

impl std::fmt::Display for OptionValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionValue::Bool(b) => f.write_str(if *b { "1" } else { "0" }),
            OptionValue::Int(i)  => f.write_str(&i.to_string()),
            OptionValue::ClipMasked(c) => f.write_str(match c {
                ClipMasked::None           => "0",
                ClipMasked::MultipatchOnly => "1",
                ClipMasked::All            => "2"
            }),
            OptionValue::TexWidthClamp(t) => f.write_str(match t {
                TexWidthClamp::All            => "0",
                TexWidthClamp::SolidWallsOnly => "1",
                TexWidthClamp::None            => "2"
            }),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Options {
    // we want a consistent ordering of the keys
    options: BTreeMap<CompOption, OptionValue>
}

impl PartialEq for Options {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Options {
    pub fn set_executable(&mut self, exe: Executable) {
        for variant in CompOption::iter() {
            if variant.min_exe() > exe || variant.max_exe() < exe {
                self.options.remove(&variant);
            }
        }
    }

    pub fn add_option(&mut self, option: CompOption, exe: Option<Executable>) {
        if let Some(value) = option.default_value(exe) {
            self.options.insert(option, value);
        }
    }

    pub fn remove_option(&mut self, option: CompOption) {
        self.options.remove(&option);
    }

    pub fn has_option(&self, option: CompOption) -> bool {
        self.options.contains_key(&option)
    }

    pub fn set_option(&mut self, option: CompOption, value: OptionValue) {
        self.options.insert(option, value);
    }
}

impl<'a> IntoIterator for &'a Options {
    type Item = (&'a CompOption, &'a OptionValue);
    type IntoIter = std::collections::btree_map::Iter<'a, CompOption, OptionValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.options.iter()
    }
}


impl serde::Serialize for Options {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut result = String::new();

        for (option, value) in &self.options {
            if !result.is_empty() {
                result.push('\n');
            }

            write!(&mut result, "{option} {value}").map_err(serde::ser::Error::custom)?;
        }

        serializer.serialize_str(&result)
    }
}

impl<'a> serde::Deserialize<'a> for Options {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'a> {
        let s = String::deserialize(deserializer)?;
        let mut options = Options::default();

        for line in s.lines() {
            let mut parts = line.split_whitespace();
            if line.split_whitespace().count() != 2 {
                return Err(serde::de::Error::custom(format!("Invalid options line: {line}")));
            }
            if let (Some(opt), Some(value)) = (parts.next(), parts.next()) {
                let value: u16 = value.parse().map_err(serde::de::Error::custom)?;
                match opt {
                    "comp_texwidthclamp" => {
                        options.options.insert(
                            CompOption::comp_texwidthclamp,
                            OptionValue::TexWidthClamp(
                                TexWidthClamp::from_repr(value)
                                    .ok_or(format!("Invalid value {value} for comp_texwidthclamp"))
                                    .map_err(serde::de::Error::custom)?
                        ));
                    },
                    "comp_clipmasked" => {
                        options.options.insert(
                            CompOption::comp_clipmasked,
                            OptionValue::ClipMasked(
                                ClipMasked::from_repr(value)
                                    .ok_or(format!("Invalid value {value} for comp_clipmasked"))
                                    .map_err(serde::de::Error::custom)?
                        ));
                    },
                    "player_helpers" => {
                        options.options.insert(
                            CompOption::player_helpers,
                            // TODO: check these ranges
                            OptionValue::Int(value.clamp(0, 3))
                        );
                    },
                    "friend_distance" => {
                        options.options.insert(
                            CompOption::friend_distance,
                            // TODO: check these ranges
                            OptionValue::Int(value.clamp(0, 999))
                        );
                    },
                    _ => { 
                        options.options.insert(
                            CompOption::from_str(opt).map_err(serde::de::Error::custom)?,
                            OptionValue::Bool(value != 0)
                        );
                    }
                }
            }
        }
        Ok(options)
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
    #[test]
    fn deserialize_single_option() {
        let json = r#"
            "comp_soul 0"
        "#;
        let options: Options = serde_json::from_str(json).unwrap();
        assert_eq!(options.options.len(), 1);
        assert_eq!(options.options.get(&CompOption::comp_soul),
                   Some(&OptionValue::Bool(false)));
    }
    #[test]
    fn deserialize_multiple_options() {
        let json = r#"
            "comp_soul 0\ncomp_texwidthclamp 1"
        "#;
        let options: Options = serde_json::from_str(json).unwrap();
        assert_eq!(options.options.len(), 2);
        assert_eq!(options.options.get(&CompOption::comp_soul),
                   Some(&OptionValue::Bool(false)));
        assert_eq!(options.options.get(&CompOption::comp_texwidthclamp),
                   Some(&OptionValue::TexWidthClamp(TexWidthClamp::SolidWallsOnly)));
    }
    #[test]
    fn deserialize_options_single_line() {
        let json = r#"
            "comp_soul 0 comp_texwidthclamp 1"
        "#;
        assert!(serde_json::from_str::<Options>(json).is_err());
    }
    #[test]
    fn deserialize_options_invalid_range() {
        // integer and boolean options are clamped to the valid range
        let json = r#"
            "comp_soul 2"
        "#;
        let options: Options = serde_json::from_str(json).unwrap();
        assert_eq!(options.options.get(&CompOption::comp_soul),
                   Some(&OptionValue::Bool(true)));
        let json = r#"
            "player_helpers 4"
        "#;
        let options: Options = serde_json::from_str(json).unwrap();
        assert_eq!(options.options.get(&CompOption::player_helpers),
                   Some(&OptionValue::Int(3)));
        // enum options error when the value is invalid
        let json = r#"
            "comp_texwidthclamp 3"
        "#;
        assert!(serde_json::from_str::<Options>(json).is_err());
    }
    #[test]
    fn deserialize_options_doesnt_exist() {
        let json = r#"
            "comp_fake 0"
        "#;
        assert!(serde_json::from_str::<Options>(json).is_err());
    }
    #[test]
    fn deserialize_all_enum_variants() {
        let json = r#"
            "comp_texwidthclamp 0"
        "#;
        let options: Options = serde_json::from_str(json).unwrap();
        assert_eq!(options.options.get(&CompOption::comp_texwidthclamp),
                   Some(&OptionValue::TexWidthClamp(TexWidthClamp::All)));
        let json = r#"
            "comp_texwidthclamp 1"
        "#;
        let options: Options = serde_json::from_str(json).unwrap();
        assert_eq!(options.options.get(&CompOption::comp_texwidthclamp),
                   Some(&OptionValue::TexWidthClamp(TexWidthClamp::SolidWallsOnly)));
        let json = r#"
            "comp_texwidthclamp 2"
        "#;
        let options: Options = serde_json::from_str(json).unwrap();
        assert_eq!(options.options.get(&CompOption::comp_texwidthclamp),
                   Some(&OptionValue::TexWidthClamp(TexWidthClamp::None)));
        let json = r#"
            "comp_clipmasked 0"
        "#;
        let options: Options = serde_json::from_str(json).unwrap();
        assert_eq!(options.options.get(&CompOption::comp_clipmasked),
                   Some(&OptionValue::ClipMasked(ClipMasked::None)));
        let json = r#"
            "comp_clipmasked 1"
        "#;
        let options: Options = serde_json::from_str(json).unwrap();
        assert_eq!(options.options.get(&CompOption::comp_clipmasked),
                   Some(&OptionValue::ClipMasked(ClipMasked::MultipatchOnly)));
        let json = r#"
            "comp_clipmasked 2"
        "#;
        let options: Options = serde_json::from_str(json).unwrap();
        assert_eq!(options.options.get(&CompOption::comp_clipmasked),
                   Some(&OptionValue::ClipMasked(ClipMasked::All)));
    }
}