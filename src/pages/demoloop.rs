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
use crate::id24json::{ID24Json, ID24JsonData};
use crate::id24json::demoloop::{Entry, DemoType, OutRowWipe};
use crate::widgets::aligned_row;

#[derive(Debug, Clone)]
pub enum Message {
    NewEntry,
    SelectEntry(Option<usize>),
    ChangeDemoType(DemoType),
    ChangeOutRowWipe(OutRowWipe),
    ChangeDuration(f32),
}

#[derive(Default)]
pub struct Page {
    index: Option<usize>,
}

impl Page {
    pub fn view<'a>(&'a self, json: &'a ID24Json) -> Element<'a, Message> {
        if let ID24JsonData::DEMOLOOP { entries } = &json.data {
            let mut properties_list = Vec::new();
            if let Some(idx) = self.index {
                let Entry {
                    primarylump,
                    secondarylump,
                    duration,
                    demo_type,
                    outrowwipe
                } = &entries[idx];
                properties_list.push(aligned_row("Primary lump", widget::text_input("", primarylump)));
                if *demo_type == DemoType::ArtScreen {
                    properties_list.push(aligned_row("Music lump", widget::text_input("", secondarylump)));
                }
                let type_pick = cosmic::iced::widget::pick_list(
                    DemoType::VARIANTS,
                    Some(demo_type),
                    Message::ChangeDemoType
                );
                properties_list.push(aligned_row("Type:", type_pick));
                if *demo_type == DemoType::ArtScreen {
                    let duration_spin = widget::spin_button(
                        format!("{:.2}", duration),
                        *duration,
                        0.1, 0.0, 30.0,
                        Message::ChangeDuration
                    );
                    properties_list.push(aligned_row("Duration:", duration_spin));
                }
                let wipe_pick = cosmic::iced::widget::pick_list(
                    OutRowWipe::VARIANTS,
                    Some(outrowwipe),
                    Message::ChangeOutRowWipe
                );
                properties_list.push(aligned_row("Wipe type:", wipe_pick));
            }

            let properties_list = properties_list.into_iter().fold(
                widget::list_column(),
                widget::ListColumn::add
            );

            let entries_list = entries.iter().enumerate().fold(
                widget::list_column(),
                |acc, (idx, _)|
                    acc.add(widget::button::text(idx.to_string())
                        .on_press(Message::SelectEntry(Some(idx)))
                        .width(Length::Fill)
                        .class(match self.index {
                            Some(i) if i == idx => widget::button::ButtonClass::Suggested,
                            _ => widget::button::ButtonClass::Text
                        }))
                );

            let entries_list = entries_list.add(widget::button::text("New Entry")
                .on_press(Message::NewEntry)
                .width(Length::Fill)
                .class(widget::button::ButtonClass::Text));

            let content = widget::row::with_children(vec![
                widget::container(widget::scrollable(properties_list))
                    .width(Length::FillPortion(2))
                    .into(),
                widget::divider::vertical::heavy().into(),
                widget::container(widget::scrollable(entries_list))
                    .width(Length::FillPortion(1))
                    .into(),
            ]);

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
            Message::NewEntry => {
                if let ID24JsonData::DEMOLOOP { entries } = &mut json.data {
                    entries.push(Entry::default());
                }
            },
            Message::ChangeDemoType(demo_type) => {
                if let (ID24JsonData::DEMOLOOP { entries }, Some(idx)) = (&mut json.data, self.index) {
                    entries[idx].demo_type = demo_type;
                }
            },
            Message::ChangeOutRowWipe(outrowwipe) => {
                if let (ID24JsonData::DEMOLOOP { entries }, Some(idx)) = (&mut json.data, self.index) {
                    entries[idx].outrowwipe = outrowwipe;
                }
            },
            Message::ChangeDuration(duration) => {
                if let (ID24JsonData::DEMOLOOP { entries }, Some(idx)) = (&mut json.data, self.index) {
                    entries[idx].duration = duration;
                }
            },
            Message::SelectEntry(idx) => {
                self.index = idx;
            },
        }
        Task::none()
    }
}
