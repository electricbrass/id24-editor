struct SelectList {
    
}

// TODO: make a widget to replace the lists being used for skydefs and flatmappings
// segmented_control is visually nice but not really intended for this purpose and adds a lot of complexity
// all we really need is something that works like the current list but with better styling
// and a way to set a specific button to display as selected

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
