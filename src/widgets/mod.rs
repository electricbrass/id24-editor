use cosmic::{widget, Element};
use cosmic::iced::Alignment;

pub fn aligned_row<'a, Message: 'a>(
    label: &'a str,
    widget: impl Into<Element<'a, Message>>,
) -> widget::Row<'a, Message> {
    widget::row()
        .push(widget::text::heading(label))
        .push(widget::horizontal_space())
        .push(widget.into())
        .align_y(Alignment::Center)
}