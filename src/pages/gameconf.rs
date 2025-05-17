use cosmic::iced::{Alignment, Length};
use cosmic::{widget, Element, Task};
use strum::VariantArray;
use crate::id24json::{ID24Json, ID24JsonData};
use crate::id24json::gameconf::{Executable, Mode, Options, CompOption, OptionValue, TexWidthClamp, ClipMasked};
use crate::widgets::aligned_row;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateExe(Executable),
    UpdateMode(Mode),
    UpdateTitle(String),
    UpdateAuthor(String),
    UpdateVersion(String),
    UpdateIWAD(String),
    SetNewOption(CompOption),
    AddNewOption(CompOption),
    RemoveOption(CompOption),
    SetOption(CompOption, OptionValue),
    EditDescription(widget::text_editor::Action),
    Dummy, // TODO: remove this
}

#[derive(Default)]
pub struct Page {
    // TODO: figure out how to actually get the content written to the json struct
    description_content: widget::text_editor::Content,
    new_option: Option<CompOption>
}

impl Page {
    #[allow(clippy::too_many_lines)]
    pub fn view<'a>(&'a self, json: &'a ID24Json) -> Element<'a, Message> {
        if let ID24JsonData::GAMECONF {
            title, author, version,
            iwad, pwadfiles, dehfiles,
            executable, mode, options,
            playertranslations, wadtranslation, ..
        } = &json.data {
            let title_input = widget::text_input(
                "my cool wad",
                title.as_deref().unwrap_or(""))
                .on_input(Message::UpdateTitle);
            let author_input = widget::text_input(
                "electricbrass",
                author.as_deref().unwrap_or(""))
                .on_input(Message::UpdateAuthor);
            let version_input = widget::text_input(
                "1.0",
                version.as_deref().unwrap_or(""))
                .on_input(Message::UpdateVersion);
            let iwad_input = widget::text_input(
                "doom2.wad",
                iwad.as_deref().unwrap_or(""))
                .on_input(Message::UpdateIWAD);
            let description_input = widget::text_editor(&self.description_content)
                .placeholder("a really awesome set of levels")
                .on_action(Message::EditDescription);
            let exe_pick = cosmic::iced::widget::pick_list(
                Executable::VARIANTS,
                executable.as_ref(),
                Message::UpdateExe
            ).placeholder("None");
            let mode_pick = cosmic::iced::widget::pick_list(
                Mode::VARIANTS,
                mode.as_ref(),
                Message::UpdateMode
            ).placeholder("None");
            let options_list = CompOption::VARIANTS
                .iter()
                .copied()
                .filter(|opt|
                    // TODO: clean this up so its not one big hard to read condition
                    (options.is_none() || !options.as_ref().unwrap().has_option(*opt)) &&
                    (executable.is_none() ||
                    (opt.max_exe() >= executable.unwrap() &&
                     opt.min_exe() <= executable.unwrap())))
                .collect::<Vec<CompOption>>();
            let option_pick = cosmic::iced::widget::pick_list(
                options_list,
                self.new_option,
                Message::SetNewOption
            ).placeholder("None");

            let options_list = if let Some(options) = options {
                options.into_iter()
                    .map(|(option, value)| {
                        let (short, long) = option.description();
                        let row = widget::row()
                            .push(
                                widget::button::text("Delete")
                                    .on_press(Message::RemoveOption(*option))
                            )
                            .push(widget::tooltip(
                                widget::text::heading(short), long,
                                widget::tooltip::Position::FollowCursor
                            ))
                            .push(widget::horizontal_space())
                            .align_y(Alignment::Center);
                        match value {
                            OptionValue::Bool(b) => row.push(
                                widget::toggler(*b)
                                    .on_toggle(|b| Message::SetOption(*option, OptionValue::Bool(b)))
                            ),
                            OptionValue::Int(i) => row.push(
                                widget::spin_button(
                                    i.to_string(), *i,
                                    1, 0, 3, // TODO: get these values from somewhere
                                    {
                                        let option = *option;
                                        move |i| Message::SetOption(option, OptionValue::Int(i))
                                    }
                                )
                            ),
                            OptionValue::TexWidthClamp(t) => row.push(
                                cosmic::iced::widget::pick_list(
                                    TexWidthClamp::VARIANTS,
                                    Some(*t),
                                    |t| Message::SetOption(*option, OptionValue::TexWidthClamp(t))
                                )
                            ),
                            OptionValue::ClipMasked(c) => row.push(
                                cosmic::iced::widget::pick_list(
                                    ClipMasked::VARIANTS,
                                    Some(*c),
                                    |c| Message::SetOption(*option, OptionValue::ClipMasked(c))
                                )
                            )
                        }
                    }).collect()
            } else {
                Vec::new()
            };

            // TODO: figure out how to represent the lists of files and lumps
            // need a way to add and remove
            let list = widget::list_column()
                .add(aligned_row("Title:", title_input))
                .add(aligned_row("Author:", author_input))
                .add(aligned_row("Version:", version_input))
                .add(aligned_row("Description:", description_input))
                .add(aligned_row("IWAD File:", iwad_input))
                .add(aligned_row("Executable:", exe_pick))
                .add(aligned_row("Mode:", mode_pick))
                .add(widget::row()
                    .push(widget::text::heading("Options:"))
                    .push(widget::horizontal_space())
                    .push(
                        widget::button::text("Add Option")
                            .on_press_maybe(self.new_option.map(Message::AddNewOption))
                    )
                    .push(option_pick)
                    .align_y(Alignment::Center));

            let list = options_list
                .into_iter()
                .fold(list, widget::ListColumn::add);

            widget::container(list)
                .center_x(Length::Fill)
                .center_y(Length::Shrink)
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
            Message::UpdateTitle(t) => {
                if let ID24JsonData::GAMECONF { title, .. } = &mut json.data {
                    *title = (!t.is_empty()).then_some(t);
                }
            },
            Message::UpdateAuthor(a) => {
                if let ID24JsonData::GAMECONF { author, .. } = &mut json.data {
                    *author = (!a.is_empty()).then_some(a);
                }
            },
            Message::UpdateVersion(v) => {
                if let ID24JsonData::GAMECONF { version, .. } = &mut json.data {
                    *version = (!v.is_empty()).then_some(v);
                }
            },
            Message::UpdateIWAD(i) => {
                if let ID24JsonData::GAMECONF { iwad, .. } = &mut json.data {
                    *iwad = (!i.is_empty()).then_some(i);
                }
            },
            Message::UpdateExe(e) => {
                if let ID24JsonData::GAMECONF { executable, options, .. } = &mut json.data {
                    *executable = Some(e);
                    self.new_option = None;
                    if let Some(o) = options { o.set_executable(e) }
                }
            },
            Message::UpdateMode(m) => {
                if let ID24JsonData::GAMECONF { mode, .. } = &mut json.data {
                    *mode = Some(m);
                }
            },
            Message::SetNewOption(o) => {
                self.new_option = Some(o);
            },
            Message::AddNewOption(o) => {
                if let ID24JsonData::GAMECONF { options, .. } = &mut json.data {
                    options.get_or_insert_with(Options::default).add_option(o);
                    self.new_option = None;
                }
            },
            Message::RemoveOption(o) => {
                if let ID24JsonData::GAMECONF { options: Some(options), .. } = &mut json.data {
                    options.remove_option(o);
                }
            },
            Message::SetOption(o, v) => {
                if let ID24JsonData::GAMECONF { options: Some(options), .. } = &mut json.data {
                    options.set_option(o, v);
                }
            }
            Message::EditDescription(action) => self.description_content.perform(action),
            _ => ()
        }
        Task::none()
    }
}