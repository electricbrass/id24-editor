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

use std::fmt::{Display, Formatter};

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct Entry {
    pub primarylump: String,
    pub secondarylump: String,
    pub duration: f32,
    #[serde(rename = "type")]
    pub demo_type: DemoType,
    pub outrowwipe: OutRowWipe
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, strum_macros::VariantArray, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum DemoType {
    ArtScreen = 0,
    DemoLump = 1
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, strum_macros::VariantArray, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum OutRowWipe {
    Immediate = 0,
    ScreenMelt = 1
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            primarylump: "DEMO1".to_owned(),
            secondarylump: "D_RUNNIN".to_owned(),
            duration: 3.0,
            demo_type: DemoType::DemoLump,
            outrowwipe: OutRowWipe::ScreenMelt,
        }
    }
}

impl Display for DemoType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DemoType::ArtScreen => "Art Screen",
            DemoType::DemoLump  => "Demo Lump",
        })
    }
}

impl Display for OutRowWipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            OutRowWipe::Immediate   => "Immediate",
            OutRowWipe::ScreenMelt  => "Screen Melt",
        })
    }
}
