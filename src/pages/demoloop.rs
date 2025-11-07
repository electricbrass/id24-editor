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
use crate::id24json::{demoloop, ID24Json, ID24JsonData};
use crate::id24json::demoloop::{Entry, DemoType, OutRowWipe};
use crate::widgets::aligned_row;

#[derive(Debug, Clone)]
pub enum Message {
    SelectEntry(Option<usize>),
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
                properties_list.push(aligned_row("Secondary lump", widget::text_input("", secondarylump)));
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
        Task::none()
    }
}
