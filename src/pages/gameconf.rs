use cosmic::iced::{Alignment, Length};
use cosmic::{widget, Element, Task};
use strum::VariantArray;
use crate::id24json::{ID24Json, ID24JsonData};
use crate::id24json::gameconf::{Executable, Mode, Options, CompOption, OptionValue, TexWidthClamp, ClipMasked};
use crate::widgets::aligned_row;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateExe(Executable),
    ClearExe,
    UpdateMode(Mode),
    ClearMode,
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

            // TODO: this is slow when many options are being displayed, figure out what is the issue
            // enabling wgpu fixes this but at least on windows it has a few issues of its own
            let options_list = if let Some(options) = options {
                options.into_iter()
                    .map(|(option, value)| {
                        let (short, long) = option.description();
                        let row = widget::row()
                            .push(
                                widget::button::text("Delete")
                                    .on_press(Message::RemoveOption(*option))
                            )
                            .push(widget::horizontal_space().width(20))
                            .push(widget::tooltip(
                                widget::text::heading(short), long,
                                widget::tooltip::Position::Top
                            ).padding(20).gap(3))
                            .push(widget::horizontal_space())
                            .align_y(Alignment::Center);
                        let row = match value {
                            OptionValue::Bool(b) => row.push(
                                widget::toggler(*b)
                                    .on_toggle(|b| Message::SetOption(*option, OptionValue::Bool(b)))
                            ),
                            OptionValue::Int(i) => row.push(
                                widget::spin_button(
                                    i.to_string(), *i,
                                    1, 0, if *option == CompOption::player_helpers {3} else {999}, // TODO: don't just hardcode this here
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
                        };
                        let default_indicator = {
                            // TODO: should this indicate when option *is* or *is not* default
                            let inner: Element<Message> =
                                if executable.is_some() && Some(*value) == option.default_value(*executable) {
                                    widget::tooltip(
                                        widget::icon(widget::icon::from_svg_bytes(
                                            include_bytes!("../../res/icons/red-circle.svg"))
                                        ).size(12),
                                        "Option not modified from default",
                                        widget::tooltip::Position::Left
                                    ).padding(10).into()
                                } else {
                                    widget::horizontal_space().into()
                                };

                            widget::container(inner)
                                // TODO: figure out exactly what values are wanted here
                                .width(Length::Fixed(20.0))
                                .align_x(Alignment::End)
                                .align_y(Alignment::Start)
                        };
                        row.push(default_indicator)
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
                .add(aligned_row("Executable:", exe_pick)
                    .push(widget::button::text("Clear")
                        .on_press(Message::ClearExe)))
                .add(aligned_row("Mode:", mode_pick)
                    .push(widget::button::text("Clear")
                        .on_press(Message::ClearMode)))
                .add(widget::column()
                    .push(widget::row()
                        .push(widget::text::heading("Options:"))
                        .push(widget::horizontal_space())
                        .push(
                            widget::button::text("Add Option")
                                .on_press_maybe(self.new_option.map(Message::AddNewOption))
                        )
                        .push(option_pick)
                        .align_y(Alignment::Center))
                    // TODO: make the options list collapsable
                    .push(options_list
                      .into_iter()
                      .fold(widget::list_column(), widget::ListColumn::add)));

            widget::container(widget::scrollable(list))
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
            Message::ClearExe => {
                if let ID24JsonData::GAMECONF { executable, .. } = &mut json.data {
                    *executable = None;
                }
            },
            Message::UpdateMode(m) => {
                if let ID24JsonData::GAMECONF { mode, .. } = &mut json.data {
                    *mode = Some(m);
                }
            },
            Message::ClearMode => {
                if let ID24JsonData::GAMECONF { mode, .. } = &mut json.data {
                    *mode = None;
                }
            },
            Message::SetNewOption(o) => {
                self.new_option = Some(o);
            },
            Message::AddNewOption(o) => {
                if let ID24JsonData::GAMECONF { options, executable, .. } = &mut json.data {
                    options.get_or_insert_with(Options::default).add_option(o, *executable);
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

impl CompOption {
    // TODO: translate these (and all other text in the ui) into other languages
    #[allow(clippy::too_many_lines)]
    fn description(self) -> (&'static str, &'static str) {
        match self {
            Self::comp_soul => (
                "Lost souls don't bounce off flat surfaces",
                "this is a longer description for the option that\n\
                 might include information like the default value\n\
                 for the different executable levels\n\
                 this one should mention the difference between\n\
                 doom 1 and 2"
            ),
            Self::comp_finaldoomteleport => (
                "Use Final Doom teleport behavior",
                "mention the z thing here"
            ),
            Self::comp_texwidthclamp => (
                "Clamp texture widths to powers of 2",
                "Non-power of 2 texture widths will be\n\
                 rounded down to the nearest power of 2"
            ),
            Self::comp_clipmasked => (
                "Clip 2-sided middle textures",
                "lorem ipsum"
            ),
            Self::comp_thingfloorlight => (
                "Light things based on floor lighting",
                "If enabled, things are affected by floor\n\
                 light transfers. If disabled, things are\n\
                 always lit by the sector light."
            ),
            Self::comp_musinfo => (
                "Enable MUSINFO",
                "lorem ipsum"
            ),
            Self::comp_moveblock => (
                "Use vanilla movement clipping code",
                "Effects of this include mancubus\n\
                 fireballs clipping through some walls"
            ),
            Self::weapon_recoil => (
                "Push player back when firing weapons",
                "lorem ipsum"
            ),
            Self::monsters_remember => (
                "friendly monsters remember targets idk",
                "lorem ipsum"
            ),
            Self::monster_infighting => (
                "Enable monster infighting",
                "lorem ipsum"
            ),
            Self::monster_backing => (
                "Ranged monsters back away from melee targets",
                "lorem ipsum"
            ),
            Self::monster_avoid_hazards => (
                "Monsters avoid environmental hazards",
                "this includes crushing ceilings,\n\
                 but what about damaging sectors? idk"
            ),
            Self::monkeys => (
                "Monsters can climb steep stairs",
                "banana heehoo"
            ),
            Self::monster_friction => (
                "make friction affect monsters",
                "lorem ipsum"
            ),
            Self::help_friends => (
                "monsters help dying friends",
                "lorem ipsum"
            ),
            Self::player_helpers => (
                "# of helper dogs",
                "woof woof"
            ),
            Self::friend_distance => (
                "friend distance",
                "units are map units i'd assume"
            ),
            Self::dog_jumping => (
                "Allow helper dogs to jump from high ledges",
                "woof woof"
            ),
            Self::comp_telefrag => (
                "Allow all monsters to telefrag on MAP30",
                "lorem ipsum"
            ),
            Self::comp_dropoff => (
                "Prevent enemies from walking off ledges",
                "deprecated in favor of comp_ledgeblock?"
            ),
            Self::comp_vile => (
                "Allow Arch-viles to create ghosts",
                "lorem ipsum"
            ),
            Self::comp_pain => (
                "Prevent Pain Elementals from spawning over the Lost Soul limit",
                "lorem ipsum"
            ),
            Self::comp_skull => (
                "Allow lost souls to spawn past impassable lines",
                "lorem ipsum"
            ),
            Self::comp_blazing => (
                "blazing door double sounds",
                "lorem ipsum"
            ),
            Self::comp_doorlight => (
                "abrupt door lighting changes?",
                "what does this even mean"
            ),
            Self::comp_model => (
                "Use vanilla linedef trigger model",
                "what behavior does this result in?"
            ),
            Self::comp_god => (
                "Use vanilla IDDQD behavior",
                "God mode is disabled if the player\n\
                 enters a sector with special 11.\n\
                 Damage over 1000 is not prevented."
            ),
            Self::comp_falloff => (
                "dont pull hanging monsters off ledges",
                "lorem ipsum"
            ),
            Self::comp_floors => (
                "Use vanilla floor movement",
                "Moving sectors are block when\n\
                 containing things that touch walls\n\
                 or ceilings"
            ),
            Self::comp_skymap => (
                "Don't apply invulnerability affect to skies",
                "lorem ipsum"
            ),
            Self::comp_pursuit => (
                "monsters can infight immediately",
                "disable the annoying mbf thing"
            ),
            Self::comp_doorstuck => (
                "monsters get stuck on door tracks",
                "lorem ipsum"
            ),
            Self::comp_staylift => (
                "monsters randomly walk off lifts their target is on",
                "disable to keep them staying on the lifts"
            ),
            Self::comp_zombie => (
                "Allow dead players to activate linedefs",
                "lorem ipsum"
            ),
            Self::comp_stairs => (
                "use vanilla stairbuilder bugs i think?",
                "lorem ipsum"
            ),
            Self::comp_infcheat => (
                "Infinite duration for IDBEHOLD powerups",
                "makes em toggleable :)"
            ),
            Self::comp_zerotags => (
                "Allow linedef actions with tag 0",
                "lorem ipsum"
            ),
            Self::comp_respawn => (
                "Respawn icon of sin monsters at 0,0",
                "lorem ipsum"
            ),
            Self::comp_ledgeblock => (
                "monsters are blocked by ledges except when scrolling",
                "lorem ipsum"
            ),
            Self::comp_friendlyspawn => (
                "spawned things inherit friendly flag",
                "lorem ipsum"
            ),
            Self::comp_voodooscroller => (
                "smth about voodoo doll scroll speed",
                "lorem ipsum"
            ),
            Self::comp_reservedlineflag => (
                "line flag 0x0800 disabled extended flags",
                "wtf does this mean"
            ),
            Self::comp_666 => (
                "Use pre-Ultimate Doom boss death checks",
                "lorem ipsum"
            ),
            Self::comp_maskedanim => (
                "Disable animations for 2-sided midtextures",
                "Doom v1.666 behavior?"
            ),
            Self::comp_ouchface => (
                "Use vanilla OUCHFACE behavior",
                "why does anyone care"
            ),
            Self::comp_maxhealth => (
                "Only apply DeHackEd \"Max Health\" to health bonuses",
                "lorem ipsum"
            ),
            Self::comp_sound => (
                "use sound errors?",
                "lorem ipsum"
            ),
        }
    }
}