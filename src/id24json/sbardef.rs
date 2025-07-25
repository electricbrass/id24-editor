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

use super::{serialize_vec_as_null, serialize_vec_non_empty};

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct NumberFont {
    name: String,
    #[serde(rename = "type")]
    numberfont_type: NumberFontType,
    stem: String
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Clone, PartialEq, Debug)]
#[repr(u8)]
enum NumberFontType {
    MonoSpacedZero = 0,
    MonoSpaceWidest = 1,
    Proportional = 2
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct StatusBar {
    height: u16,
    fullscreenrender: bool,
    fillflat: Option<String>, // spec says that this can't be null, but it is in LoR :/
    #[serde(serialize_with = "serialize_vec_as_null")]
    children: Option<Vec<SBarElem>> // other children arrays can be null according to spec, but not this one...of course it is in LoR
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
struct SBarElem {
    // TODO: spec says all these can be undefined but does *not* say they can be null, currently this works for writing but when reading this will allow null to be accepted
    // I suppose it's maybe okay if we let bad json be fixed up a bit
    // Already any extra fields not part of the spec will just be thrown away
    #[serde(skip_serializing_if = "Option::is_none")]
    canvas: Option<Canvas>,
    #[serde(skip_serializing_if = "Option::is_none")]
    graphic: Option<Graphic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    face: Option<Face>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facebackground: Option<FaceBG>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<Percent>
}

type Face = Canvas;
type FaceBG = Canvas;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
struct Canvas {
    x: i16,
    y: i16,
    alignment: Alignment,
    #[serde(serialize_with = "serialize_vec_as_null")]
    conditions: Option<Vec<Condition>>,
    #[serde(serialize_with = "serialize_vec_as_null")]
    children: Option<Vec<SBarElem>>
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
struct Graphic {
    x: i16,
    y: i16,
    alignment: Alignment,
    tranmap: Option<String>,
    translation: Option<String>,
    #[serde(serialize_with = "serialize_vec_as_null")]
    conditions: Option<Vec<Condition>>,
    #[serde(serialize_with = "serialize_vec_as_null")]
    children: Option<Vec<SBarElem>>,
    patch: String
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
struct Animation {
    x: i16,
    y: i16,
    alignment: Alignment,
    tranmap: Option<String>,
    translation: Option<String>,
    #[serde(serialize_with = "serialize_vec_as_null")]
    conditions: Option<Vec<Condition>>,
    #[serde(serialize_with = "serialize_vec_as_null")]
    children: Option<Vec<SBarElem>>,
    #[serde(serialize_with = "serialize_vec_non_empty")]
    frames: Vec<Frame>
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
struct Frame {
    lump: String,
    duration: f32
}

type Percent = Number;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
struct Number {
    x: i16,
    y: i16,
    alignment: Alignment,
    tranmap: Option<String>,
    translation: Option<String>,
    #[serde(serialize_with = "serialize_vec_as_null")]
    conditions: Option<Vec<Condition>>,
    #[serde(serialize_with = "serialize_vec_as_null")]
    children: Option<Vec<SBarElem>>,
    font: String,
    #[serde(rename = "type")]
    num_type: NumberType,
    param: u8,
    maxlength: u8
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Clone, PartialEq, Debug)]
#[repr(u8)]
enum NumberType {
    // TODO: maybe make the ammo names clearer
    Health = 0,
    Armor = 1,
    Frags = 2,
    AmmoParam = 3,
    AmmoCurrWeapon = 4,
    MaxAmmoParam = 5,
    AmmoParamWeapon = 6,
    MaxAmmoParamWeapon = 7
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
struct Condition {
    condition: ConditionType,
    param: u8
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Clone, PartialEq, Debug)]
#[repr(u8)]
enum ConditionType {
    WeaponOwned           = 0, // Whether the weapon defined by param is owned
    WeaponSelected        = 1, // Whether the weapon defined by param is selected
    WeaponNotSelected     = 2, // Whether the weapon defined by param is not selected
    WeaponValidAmmo       = 3, // Whether the weapon defined by param has a valid ammo type
    CurrWeaponValidAmmo   = 4, // Whether the selected weapon has a valid ammo type
    MatchesCurrWeaponAmmo = 5, // Whether the ammo type defined by param matches the selected weapon’s ammo type
    AnyWeaponOwned        = 6, // Whether any weapon in a slot defined by param is owned
    AnyWeaponNotOwned     = 7, // Whether any weapon in a slot defined by param not owned
    AnyWeaponSelected     = 8, // Whether any weapon in a slot defined by param is selected
    AnyWeaponNotSelected  = 9, // Whether any weapon in a slot defined by param is not selected
    ItemOwned             = 10, // Whether the item defined by param is owned
    ItemNotOwned          = 11, // Whether the item defined by param is not owned
    GameVersionGreaterEq  = 12, // Whether the current game version is greater than or equal to the feature level defined by param
    GameVersionLess       = 13, // Whether the current game version is less than the feature level defined by param
    SessionTypeEqual      = 14, // Whether the session type is equal to the type defined by param
    SessionTypeNotEqual   = 15, // Whether the session type is not equal to the type defined by param
    GameModeEqual         = 16, // Whether the game mode is equal to the mode defined by param
    GameModeNotEqual      = 17, // Whether the game mode is not equal to the mode defined by param
    HudModeEqual          = 18 // Whether the hud mode is equal to the mode defined by param
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Alignment {
    horizontal: HoriAlign,
    vertical: VertAlign
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum VertAlign {
    Top,
    Center,
    Bottom,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum HoriAlign {
    Left,
    Center,
    Right,
}

impl serde::Serialize for Alignment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_u8(self.to_u8())
    }
}

impl<'a> serde::Deserialize<'a> for Alignment {
    fn deserialize<D>(deserializer: D) -> Result<Alignment, D::Error> where D: serde::Deserializer<'a> {
        let value = u8::deserialize(deserializer)?;
        Alignment::from_u8(value).map_err(serde::de::Error::custom)
    }
}

impl Alignment {
    fn from_u8(value: u8) -> Result<Self, &'static str> {
        let horizontal = match value & 0b11 {
            0b00 => HoriAlign::Left,
            0b01 => HoriAlign::Center,
            0b10 => HoriAlign::Right,
            _ => return Err("Multiple horizontal alignments specified"),
        };
        let vertical = match value & 0b1100 {
            0b0000 => VertAlign::Top,
            0b0100 => VertAlign::Center,
            0b1000 => VertAlign::Bottom,
            _ => return Err("Multiple horizontal alignments specified"),
        };
        Ok(Self {
            horizontal,
            vertical
        })
    }

    fn to_u8(&self) -> u8 {
        let horizontal_bits = match self.horizontal {
            HoriAlign::Left   => 0b00,
            HoriAlign::Center => 0b01,
            HoriAlign::Right  => 0b10
        };
        let vertical_bits = match self.vertical {
            VertAlign::Top    => 0b0000,
            VertAlign::Center => 0b0100,
            VertAlign::Bottom => 0b1000
        };

        horizontal_bits | vertical_bits
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::*;
    #[test]
    fn read_sbardef() {
        let json = r#"{
            "type": "statusbar",
            "version": "1.0.0",
            "metadata": null,
            "data":
            {
                "numberfonts":
                [
                    {
                        "name": "BigRed",
                        "type": 0,
                        "stem": "STT"
                    },
                    {
                        "name": "SmallGrey",
                        "type": 1,
                        "stem": "STG"
                    },
                    {
                        "name": "SmallYellow",
                        "type": 2,
                        "stem": "STYS"
                    }
                ],
                "statusbars":
                [
                    {
                        "height": 32,
                        "fullscreenrender": false,
                        "fillflat": null,
                        "children": null
                    }
                ]
            }
        }"#;
        let data: ID24Json = serde_json::from_str(json).unwrap();
        assert_eq!(data.version, ID24JsonVersion { major: 1, minor: 0, revision: 0 });
        assert_eq!(data.data, ID24JsonData::SBARDEF {
            numberfonts: vec![
                NumberFont {
                    name: "BigRed".to_owned(),
                    numberfont_type: NumberFontType::MonoSpacedZero,
                    stem: "STT".to_owned()
                },
                NumberFont {
                    name: "SmallGrey".to_owned(),
                    numberfont_type: NumberFontType::MonoSpaceWidest,
                    stem: "STG".to_owned()
                },
                NumberFont {
                    name: "SmallYellow".to_owned(),
                    numberfont_type: NumberFontType::Proportional,
                    stem: "STYS".to_owned()
                },
            ],
            statusbars: vec![
                StatusBar {
                    height: 32,
                    fullscreenrender: false,
                    fillflat: None,
                    children: None
                }
            ]
        });
        // TODO: add tests for all the children element types, putting them all into this one would be huge
    }
}