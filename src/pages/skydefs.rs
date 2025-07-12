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

use cosmic::prelude::*;
use cosmic::iced::Length;
use cosmic::widget;
use strum::VariantArray;
use crate::id24json::{skydefs, ID24Json, ID24JsonData};
use crate::id24json::skydefs::{Fire, Sky, SkyTex, SkyType};
use crate::widgets::aligned_row;

#[derive(Default)]
pub struct Page {
    skydefs_index: SkydefsIndex,
    skies_model: widget::segmented_button::SingleSelectModel,
    flatmapping_model: widget::segmented_button::SingleSelectModel,
}

#[derive(PartialEq, Clone, Copy, Debug, Default)]
enum SkydefsIndex {
    #[default]
    None,
    Sky(usize),
    Flatmapping(usize)
}

#[derive(Debug, Clone)]
pub enum SkyTexMessage {
    ChangeName(String),
    ChangeMid(u16),
    ChangeScrollX(f32),
    ChangeScrollY(f32),
    ChangeScaleX(f32),
    ChangeScaleY(f32),
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateSkyTexProp(SkyTexMessage),
    UpdateSkyTexPropFG(SkyTexMessage),
    ChangeSkyType(SkyType),
    ChangeFireSpeed(f32),
    NewSky,
    NewFlatmapping,
    DeleteSky(usize),
    DeleteFlatmapping(usize),
    SelectSky(Option<usize>),
    SelectFlatmapping(Option<usize>),
    Dummy // TODO: remove this
}

impl Page {
    pub fn reset_index(&mut self) {
        self.skydefs_index = SkydefsIndex::None;
    }

    #[allow(clippy::too_many_lines)]
    // TODO: make this less huge, just dont want it to yell at me for just a bit longer
    pub fn view<'a>(&'a self, json: &'a ID24Json) -> Element<'a, Message> {
        if let ID24JsonData::SKYDEFS { skies, flatmapping } = &json.data {
            let mut properties_list = Vec::new();
            if let (Some(skies), SkydefsIndex::Sky(idx)) = (skies, self.skydefs_index) {
                // TODO: use .get and check that the sky exists
                // we shouldn't run into a case where the index is out of bounds but just in case
                let Sky {
                    backgroundtex,
                    sky_type,
                    fire,
                    foregroundtex
                } = &skies[idx];
                macro_rules! tex_fields {
                    ($tex:expr, $message:expr, $prefix:expr) => {
                        let name_input = widget::text_input("SKY1", &$tex.name)
                                .on_input(|s| $message(SkyTexMessage::ChangeName(s)));
                        let mid_spin = widget::spin_button(
                            $tex.mid.to_string(), $tex.mid,
                            1, 0, 1024,
                            |v| $message(SkyTexMessage::ChangeMid(v))
                        );
                        let scrollx_spin = widget::spin_button(
                            $tex.scrollx.to_string(), $tex.scrollx,
                            0.1, -100.0, 100.0,
                            |v| $message(SkyTexMessage::ChangeScrollX(v))
                        );
                        let scrolly_spin = widget::spin_button(
                            $tex.scrolly.to_string(), $tex.scrolly,
                            0.1, -100.0, 100.0,
                            |v| $message(SkyTexMessage::ChangeScrollY(v))
                        );
                        let scalex_spin = widget::spin_button(
                            $tex.scalex.to_string(), $tex.scalex,
                            0.1, 0.0, 100.0,
                            |v| $message(SkyTexMessage::ChangeScaleX(v))
                        );
                        let scaley_spin = widget::spin_button(
                            $tex.scaley.to_string(), $tex.scaley,
                            0.1, 0.0, 100.0,
                            |v| $message(SkyTexMessage::ChangeScaleY(v))
                        );

                        properties_list.push(aligned_row(concat!($prefix, "Texture:"), name_input));
                        properties_list.push(aligned_row(concat!($prefix, "Mid:"), mid_spin));
                        properties_list.push(aligned_row(concat!($prefix, "Scroll X (seconds):"), scrolly_spin));
                        properties_list.push(aligned_row(concat!($prefix, "Scroll Y (seconds):"), scrollx_spin));
                        properties_list.push(aligned_row(concat!($prefix, "Scale X:"), scalex_spin));
                        properties_list.push(aligned_row(concat!($prefix, "Scale Y:"), scaley_spin));
                    };
                }

                tex_fields!(backgroundtex, Message::UpdateSkyTexProp, "");
                let type_pick = cosmic::iced::widget::pick_list(
                    SkyType::VARIANTS,
                    Some(sky_type),
                    Message::ChangeSkyType
                );
                properties_list.push(aligned_row("Type:", type_pick));
                match (sky_type, fire, foregroundtex) {
                    (SkyType::WithForeground, _, Some(foregroundtex)) => {
                        tex_fields!(foregroundtex, Message::UpdateSkyTexPropFG, "Foreground ");
                    }
                    (SkyType::Fire, Some(Fire {
                                             updatetime,
                                             palette }), _) => {
                        let time_spin = widget::spin_button(
                            (*updatetime).to_string(), *updatetime,
                            0.1, 0.0, 100.0,
                            Message::ChangeFireSpeed
                        );
                        properties_list.push(aligned_row("Animation Speed (seconds):", time_spin));
                        // TODO: figure out how this should even work in the gui and add a preview for fire skies
                        // what palette to use for when loading from standalone json? this is a time where making this part of slade wouldve been nice
                        properties_list.push(aligned_row("Palette:", widget::text::heading("coming soon")));
                    }
                    _ => ()
                }
            } else if let (Some(flatmapping), SkydefsIndex::Flatmapping(idx)) = (flatmapping, self.skydefs_index) {
                let skydefs::FlatMapping { flat, sky } = &flatmapping[idx];
                let flat_input = widget::text_input("F_SKY1", flat)
                    .on_input(|s| Message::Dummy);
                let sky_input = widget::text_input("SKY1", sky)
                    .on_input(|s| Message::Dummy);
                properties_list.push(aligned_row("Flat:", flat_input));
                properties_list.push(aligned_row("Sky:", sky_input));
            }
            let properties_list = properties_list.into_iter().fold(
                widget::list_column(),
                widget::ListColumn::add
            );
            let skies_list = skies.as_ref().map_or(
                widget::list_column(),
                |s| s.iter().enumerate().fold(
                    widget::list_column(),
                    |acc, (idx, sky)|
                        acc.add(widget::button::text(&sky.backgroundtex.name)
                            .on_press(Message::SelectSky(Some(idx)))
                            .width(Length::Fill)
                            .class(match self.skydefs_index {
                                SkydefsIndex::Sky(i) if i == idx => widget::button::ButtonClass::Suggested,
                                _ => widget::button::ButtonClass::Text
                            }))
                ));
            let flatmapping_list = flatmapping.as_ref().map_or(
                widget::list_column(),
                |s| s.iter().enumerate().fold(
                    widget::list_column(),
                    |acc, (idx, mapping)|
                        acc.add(widget::button::text(&mapping.flat)
                            .on_press(Message::SelectFlatmapping(Some(idx)))
                            .width(Length::Fill)
                            .class(match self.skydefs_index {
                                SkydefsIndex::Flatmapping(i) if i == idx => widget::button::ButtonClass::Suggested,
                                _ => widget::button::ButtonClass::Text
                            }))
                ));
            let content = widget::row::with_children(vec![
                widget::container(widget::scrollable(properties_list))
                    .width(Length::FillPortion(2))
                    .into(),

                widget::divider::vertical::heavy().into(),

                widget::container(
                    widget::column::with_children(vec![
                        widget::row::with_children(vec![
                            widget::button::text("New Sky").on_press(Message::NewSky).into(),
                            widget::horizontal_space().into(),
                            widget::button::text("Delete").on_press_maybe(match self.skydefs_index {
                                SkydefsIndex::Sky(size) => Some(Message::DeleteSky(size)),
                                _ => None
                            }).into(),
                        ]).into(),
                        widget::container(widget::scrollable(skies_list))
                            .height(Length::FillPortion(1))
                            .into(),
                        widget::divider::horizontal::heavy().into(),
                        widget::row::with_children(vec![
                            widget::button::text("New Flat Mapping").on_press(Message::NewFlatmapping).into(),
                            widget::horizontal_space().into(),
                            widget::button::text("Delete").on_press_maybe(match self.skydefs_index {
                                SkydefsIndex::Flatmapping(size) => Some(Message::DeleteFlatmapping(size)),
                                _ => None
                            }).into(),
                        ]).into(),
                        widget::container(widget::scrollable(flatmapping_list))
                            .height(Length::FillPortion(1))
                            .into(),
                    ]).spacing(5)
                )
                    .width(Length::FillPortion(1))
                    .into(),
            ])
                .padding(10)
                .spacing(10);

            widget::container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        } else {
            // TODO: figure out a better way to handle this
            widget::container(widget::text::heading("You shouldn't be here."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        }
    }

    pub fn update(&mut self, json: &mut ID24Json, message: Message) -> Task<cosmic::Action<Message>> {
        match message {
            Message::SelectSky(Some(idx)) => {
                self.skydefs_index = SkydefsIndex::Sky(idx);
            },
            Message::SelectFlatmapping(Some(idx)) => {
                self.skydefs_index = SkydefsIndex::Flatmapping(idx);
            },
            Message::SelectSky(None) | Message::SelectFlatmapping(None) => {
                self.skydefs_index = SkydefsIndex::None;
            },
            Message::NewSky => {
                if let ID24JsonData::SKYDEFS { skies, .. } = &mut json.data {
                    skies.get_or_insert_with(Vec::new).push(Sky::default());
                }
            },
            Message::NewFlatmapping => {
                if let ID24JsonData::SKYDEFS { flatmapping, .. } = &mut json.data {
                    flatmapping.get_or_insert_with(Vec::new).push(skydefs::FlatMapping::default());
                }
            },
            Message::DeleteSky(idx) => {
                // TODO: make this async in case the list is very large
                if let ID24JsonData::SKYDEFS { skies: Some(skies), .. } = &mut json.data {
                    self.skydefs_index = SkydefsIndex::None;
                    skies.remove(idx);
                }
            },
            Message::DeleteFlatmapping(idx) => {
                if let ID24JsonData::SKYDEFS { flatmapping: Some(flatmapping), .. } = &mut json.data {
                    self.skydefs_index = SkydefsIndex::None;
                    flatmapping.remove(idx);
                }
            },
            Message::UpdateSkyTexProp(skymessage) => {
                if let (ID24JsonData::SKYDEFS { skies: Some(skies), .. }, SkydefsIndex::Sky(idx)) = (&mut json.data, self.skydefs_index) {
                    match skymessage {
                        SkyTexMessage::ChangeName(name) => skies[idx].backgroundtex.name = name,
                        SkyTexMessage::ChangeMid(mid) => skies[idx].backgroundtex.mid = mid,
                        SkyTexMessage::ChangeScaleX(scale) => skies[idx].backgroundtex.scalex = scale,
                        SkyTexMessage::ChangeScaleY(scale) => skies[idx].backgroundtex.scaley = scale,
                        SkyTexMessage::ChangeScrollX(scroll) => skies[idx].backgroundtex.scrollx = scroll,
                        SkyTexMessage::ChangeScrollY(scroll) => skies[idx].backgroundtex.scrolly = scroll,
                    }
                }
            },
            Message::UpdateSkyTexPropFG(skymessage) => {
                if let (ID24JsonData::SKYDEFS { skies: Some(skies), .. }, SkydefsIndex::Sky(idx)) = (&mut json.data, self.skydefs_index) {
                    if let Some(foreground) = &mut skies[idx].foregroundtex {
                        match skymessage {
                            SkyTexMessage::ChangeName(name) => foreground.name = name,
                            SkyTexMessage::ChangeMid(mid) => foreground.mid = mid,
                            SkyTexMessage::ChangeScaleX(scale) => foreground.scalex = scale,
                            SkyTexMessage::ChangeScaleY(scale) => foreground.scaley = scale,
                            SkyTexMessage::ChangeScrollX(scroll) => foreground.scrollx = scroll,
                            SkyTexMessage::ChangeScrollY(scroll) => foreground.scrolly = scroll,
                        }
                    }
                }
            },
            Message::ChangeFireSpeed(speed) => {
                if let (ID24JsonData::SKYDEFS { skies: Some(skies), .. }, SkydefsIndex::Sky(idx)) = (&mut json.data, self.skydefs_index) {
                    if let Some(fire) = &mut skies[idx].fire {
                        fire.updatetime = speed;
                    }
                }
            },
            Message::ChangeSkyType(sky_type) => {
                if let (ID24JsonData::SKYDEFS { skies: Some(skies), .. }, SkydefsIndex::Sky(idx)) = (&mut json.data, self.skydefs_index) {
                    let sky = &mut skies[idx];
                    sky.sky_type = sky_type;
                    match sky_type {
                        SkyType::Standard => {
                            sky.foregroundtex = None;
                            sky.fire = None;
                        },
                        SkyType::WithForeground => {
                            sky.foregroundtex = Some(SkyTex::default());
                            sky.fire = None;
                        },
                        SkyType::Fire => {
                            sky.foregroundtex = None;
                            sky.fire = Some(Fire::default());
                        }
                    }
                }
            },
            _ => ()
        }

        Task::none()
    }
}