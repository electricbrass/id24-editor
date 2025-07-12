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

struct SelectList {
    
}

// TODO: make a widget to replace the lists being used for skydefs and flatmappings
// segmented_control is visually nice but not really intended for this purpose and adds a lot of complexity
// all we really need is something that works like the current list but with better styling
// and a way to set a specific button to display as selected

// i think now this can probable be done away with, i think the updated solution is good enough now

impl SelectList {
    fn on_select() {
        
    }
    
    fn index(idx: usize) {
        
    }
}

// fn select_list<T, F, G>(
//     items: Option<&[T]>,
//     get_label: F,
//     make_message: G,
//     current_index: &SkydefsIndex,
//     expected_variant: fn(usize) -> SkydefsIndex,
// ) -> widget::list_column::Column<Message>
// where
//     F: Fn(&T) -> String,
//     G: Fn(Option<usize>) -> Message,
// {
//     items.map_or(
//         widget::list_column(),
//         |items| items.iter().enumerate().fold(
//             widget::list_column(),
//             |acc, (idx, item)|
//                 acc.add(widget::button::text(&get_label(item))
//                     .on_press(make_message(Some(idx)))
//                     .width(Length::Fill)
//                     .class(match current_index {
//                         i if *i == expected_variant(idx) => widget::button::ButtonClass::Suggested,
//                         _ => widget::button::ButtonClass::Text
//                     }))
//         )
//     )
// }
