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

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Type {
    ArtScreen = 0,
    BunnyScroller = 1,
    CastRollCall = 2
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct Bunny {
    stitchimage: String,
    // TODO: check if u32 is the right type to use here
    overlay: u32,
    overlaycount: u32,
    overlaysound: u32,
    overlayx: u32,
    overlayy: u32
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct CastRollCall {
    castmembers: Vec<CastMember>
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
struct CastMember {
    // TODO: find out what goes here, seems to not be in the spec
}