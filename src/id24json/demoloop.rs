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

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct Entry {
    primarylump: String,
    secondarylump: String,
    duration: f32,
    #[serde(rename = "type")]
    demo_type: DemoType,
    outrowwipe: OutRowWipe
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Clone, PartialEq, Debug)]
#[repr(u8)]
enum DemoType {
    ArtScreen = 0,
    DemoLump = 1
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Clone, PartialEq, Debug)]
#[repr(u8)]
enum OutRowWipe {
    Immediate = 0,
    ScreenMelt = 1
}