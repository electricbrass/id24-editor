use cosmic::iced::{Alignment, Length};
use cosmic::{widget, Element, Task};
use strum::VariantArray;
use strum_macros::{EnumIter, VariantArray};
use crate::id24json::{ID24Json, ID24JsonData};
use crate::id24json::gameconf::{Executable, Mode};
use crate::widgets::aligned_row;

#[derive(Debug, Clone, Copy, VariantArray, EnumIter, PartialEq)]
enum CompOption {
    // options available in doom1.9
    CompSoul,
    CompFinalDoomTeleport,
    // options available in limitremoving
    CompTexWidthClamp,
    CompClipMasked,
    // options available in boom2.02
    CompThingFloorLight,
    // options available in complevel9
    CompMusinfo,
    // options available in mbf
    CompMoveBlock,
    WeaponRecoil,
    MonstersRemember,
    MonsterInfighting,
    MonsterBacking,
    MonsterAvoidHazards,
    Monkeys,
    MonsterFriction,
}

impl std::fmt::Display for CompOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CompOption::CompSoul => "comp_soul",
            CompOption::CompFinalDoomTeleport => "comp_finaldoomteleport",
            CompOption::CompTexWidthClamp => "comp_texwidthclamp",
            CompOption::CompClipMasked => "comp_clipmasked",
            CompOption::CompThingFloorLight => "comp_thingfloorlight",
            CompOption::CompMusinfo => "comp_musinfo",
            CompOption::CompMoveBlock => "comp_moveblock",
            _ => ""
        })
    }   
}

macro_rules! comp_array {
    ($name:ident extends $prev:expr, $($additional:expr),*) => {
        const $name: [CompOption; $prev.len() $(+ { let _ = $additional; 1})*] = {
            let mut arr = [$prev[0]; $prev.len() $(+ { let _ = $additional; 1})*];
            let mut i = 0;
            // Copy previous array
            while i < $prev.len() {
                arr[i] = $prev[i];
                i += 1;
            }
            // Add new elementss
            $(
                arr[i] = $additional;
                i += 1;
            )*
            arr
        };
    };
    ($name:ident, $($element:expr),+) => {
        const $name: [CompOption; 0 $(+ { let _ = $element; 1})*] = [
            $($element),+
        ];
    };
}

impl CompOption {
    comp_array!(DOOM1_9,
        Self::CompSoul,
        Self::CompFinalDoomTeleport
    );

    comp_array!(LIMITREMOVING extends Self::DOOM1_9,
        Self::CompTexWidthClamp,
        Self::CompClipMasked
    );

    comp_array!(BOOM2_02 extends Self::LIMITREMOVING,
        Self::CompThingFloorLight
    );

    comp_array!(COMPLEVEL9 extends Self::BOOM2_02,
        Self::CompMusinfo
    );

    comp_array!(MBF extends Self::COMPLEVEL9,
        Self::CompMoveBlock
    );
}


#[derive(Debug, Clone)]
pub enum Message {
    UpdateExe(Executable),
    UpdateMode(Mode),
    UpdateTitle(String),
    UpdateAuthor(String),
    UpdateVersion(String),
    UpdateIWAD(String),
    EditDescription(widget::text_editor::Action),
    Dummy, // TODO: remove this
}

#[derive(Default)]
pub struct Page {
    // TODO: figure out how to actually get the content written to the json struct
    description_content: widget::text_editor::Content,
}

impl Page {
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

            // TODO: figure out how to represent the lists of files and options
            // need a way to add and remove, and select from a list in the case of the options
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
                    .align_y(Alignment::Center));

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
                    // options.set_executable(e);
                }
            },
            Message::UpdateMode(m) => {
                if let ID24JsonData::GAMECONF { mode, .. } = &mut json.data {
                    *mode = Some(m);
                }
            },
            Message::EditDescription(action) => self.description_content.perform(action),
            _ => ()
        }
        Task::none()
    }
}