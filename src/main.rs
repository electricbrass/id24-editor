#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide the console window on Windows in release

mod id24json;

use id24json::{ID24Json, ID24JsonData, skydefs};

use std::fmt::{Display, Formatter};
use std::collections::HashMap;
use cosmic::widget;
use cosmic::widget::{menu, nav_bar};
use cosmic::widget::menu::key_bind::{KeyBind, Modifier};
use cosmic::iced::keyboard::Key;
use cosmic::iced::{Alignment, Length};

use cosmic::prelude::*;
use strum::{IntoEnumIterator, VariantArray};

// TODO: figure out how to bundle icons on windows/mac
// maybe make a pr to libcosmic for that

fn main() -> cosmic::iced::Result {
    let settings = cosmic::app::Settings::default();
    cosmic::app::run::<EditorModel>(settings, ())
}

#[derive(strum_macros::EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum LumpType {
    GAMECONF,
    DEMOLOOP,
    SBARDEF,
    SKYDEFS,
    Interlevel,
    Finale,
    TRAKINFO
}

impl Display for LumpType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            LumpType::GAMECONF   => "GAMECONF",
            LumpType::DEMOLOOP   => "DEMOLOOP",
            LumpType::SBARDEF    => "SBARDEF",
            LumpType::SKYDEFS    => "SKYDEFS",
            LumpType::Interlevel => "Interlevel",
            LumpType::Finale     => "Finale",
            LumpType::TRAKINFO   => "TRAKINFO",
        })
    }
}

impl From<&ID24JsonData> for LumpType {
    fn from(data: &ID24JsonData) -> Self {
        match data {
            ID24JsonData::GAMECONF   { .. } => LumpType::GAMECONF,
            ID24JsonData::DEMOLOOP   { .. } => LumpType::DEMOLOOP,
            ID24JsonData::SBARDEF    { .. } => LumpType::SBARDEF,
            ID24JsonData::SKYDEFS    { .. } => LumpType::SKYDEFS,
            ID24JsonData::Interlevel { .. } => LumpType::Interlevel,
            ID24JsonData::Finale     { .. } => LumpType::Finale,
            ID24JsonData::TRAKINFO   { .. } => LumpType::TRAKINFO,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Page {
    Settings
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MyMenuAction {
    Open,
    Save,
    SaveAs,
    Quit
}

impl menu::Action for MyMenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MyMenuAction::Open   => Message::MenuOpen,
            MyMenuAction::Save   => Message::MenuSave,
            MyMenuAction::SaveAs => Message::MenuSaveAs,
            MyMenuAction::Quit   => Message::Quit,
        }
    }
}


struct EditorModel {
    core: cosmic::Core,
    key_binds: HashMap<KeyBind, MyMenuAction>,
    nav: nav_bar::Model,
    nav_ids: HashMap<LumpType, nav_bar::Id>,
    text_content: widget::text_editor::Content,
    skydefs_index: SkydefsIndex,
    toasts: widget::Toasts<Message>,
    error_status: Option<String>,
    current_file: Option<url::Url>,
    json: ID24Json
}

#[derive(PartialEq, Clone, Copy, Debug, Default)]
enum SkydefsIndex {
    #[default]
    None,
    Sky(usize),
    Flatmapping(usize)
}

#[derive(Debug, Clone)]
enum SkyTexMessage {
    ChangeName(String),
    ChangeMid(u16),
    ChangeScrollX(f32),
    ChangeScrollY(f32),
    ChangeScaleX(f32),
    ChangeScaleY(f32),
}

#[derive(Debug, Clone)]
enum Message {
    // TODO: split each editor into its own module with its own message type
    UpdateSkyTexProp(SkyTexMessage),
    UpdateSkyTexPropFG(SkyTexMessage),
    ChangeSkyType(skydefs::SkyType),
    ChangeFireSpeed(f32),
    NewSky,
    NewFlatmapping,
    DeleteSky(usize),
    DeleteFlatmapping(usize),
    SelectSky(Option<usize>),
    SelectFlatmapping(Option<usize>),
    EditText(widget::text_editor::Action),
    InitJSON(LumpType),
    LoadJSON(Box<ID24Json>),
    CloseToast(widget::ToastId),
    MenuOpen,
    MenuSave,
    MenuSaveAs,
    Open(url::Url),
    Save(url::Url),
    Quit,
    CloseError,
    Error(String),
    ErrorConsole(String),
    Dummy
}

impl cosmic::Application for EditorModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    // TODO: make a real app id
    const APP_ID: &'static str = "placeholder_appid";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(core: cosmic::Core, flags: Self::Flags) -> (Self, cosmic::app::Task<Self::Message>) {
        let mut nav = nav_bar::Model::default();
        let mut nav_ids = HashMap::new();
        let add_type_to_nav = |lump: LumpType| {
            nav_ids.insert(lump, nav.insert().text(lump.to_string()).data::<LumpType>(lump).id());
        };

        LumpType::iter().for_each(add_type_to_nav);

        nav.insert()
            .divider_above(true)
            .text("Settings")
            .data::<Page>(Page::Settings);

        let mut app = EditorModel {
            core,
            nav,
            nav_ids,
            key_binds: HashMap::from([
                // TODO: figure out why keybinds dont work and fix display of Ctrl+Shift+S being cut off
                (KeyBind { modifiers: vec![Modifier::Ctrl], key: Key::Character("o".into()) }, MyMenuAction::Open),
                (KeyBind { modifiers: vec![Modifier::Ctrl], key: Key::Character("s".into()) }, MyMenuAction::Save),
                (KeyBind { modifiers: vec![Modifier::Ctrl, Modifier::Shift], key: Key::Character("s".into()) }, MyMenuAction::SaveAs),
                (KeyBind { modifiers: vec![Modifier::Ctrl], key: Key::Character("q".into()) }, MyMenuAction::Quit),
            ]),
            text_content: widget::text_editor::Content::new(),
            skydefs_index: SkydefsIndex::default(),
            toasts: widget::Toasts::new(Message::CloseToast),
            error_status: None,
            current_file: None,
            json: ID24Json::default()
        };
        app.set_header_title("ID24 JSON Editor".to_owned());
        let command = app.set_window_title("ID24 JSON Editor".to_owned());
        (app, command)
    }

    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            menu::root("File"),
            menu::items(
                &self.key_binds,
                vec![
                    menu::Item::Button("Open", None, MyMenuAction::Open),
                    menu::Item::Button("Save", None, MyMenuAction::Save),
                    menu::Item::Button("Save As", None, MyMenuAction::SaveAs),
                    menu::Item::Button("Quit", None, MyMenuAction::Quit)
                ],
            ),
        )]);

        vec![menu_bar.into()]
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<cosmic::Action<Message>> {
        self.nav.activate(id);
        if let Some(lump) = self.nav.data::<LumpType>(id) {
            self.skydefs_index = SkydefsIndex::None;
            return self.update(Message::InitJSON(*lump));
        }
        Task::none()
    }

    #[allow(clippy::too_many_lines)]
    // TODO: split this up, just dont want it to yell at me for just a bit longer
    fn update(&mut self, message: Self::Message) -> cosmic::app::Task<Self::Message> {
        match message {
            Message::MenuOpen => {
                return cosmic::task::future(async {
                    use cosmic::dialog::file_chooser;
                    let filter = file_chooser::FileFilter::new("JSON Files").extension("json");
                    let dialog = file_chooser::open::Dialog::new()
                        .filter(filter);
                    match dialog.open_file().await {
                        Ok(response) => Message::Open(response.url().to_owned()),
                        Err(file_chooser::Error::Cancelled) => Message::ErrorConsole("File dialog closed".to_owned()),
                        Err(why) => Message::Error(why.to_string()),
                    }
                });
            },
            Message::MenuSave => {
                return self.update(match &self.current_file {
                    Some(url) => Message::Save(url.to_owned()),
                    None => Message::MenuSaveAs
                })
            },
            Message::MenuSaveAs => {
                return cosmic::task::future(async {
                    use cosmic::dialog::file_chooser;
                    let filter = file_chooser::FileFilter::new("JSON Files").extension("json");
                    let dialog = file_chooser::save::Dialog::new()
                        .filter(filter);
                    match dialog.save_file().await {
                        Ok(response) => match response.url() {
                            Some(url) => Message::Save(url.to_owned()),
                            None => Message::ErrorConsole("No file found".to_owned()),
                        },
                        Err(file_chooser::Error::Cancelled) => Message::ErrorConsole("File dialog closed".to_owned()),
                        Err(why) => Message::Error(why.to_string()),
                    }
                });
            },
            Message::Open(url) => {
                self.current_file = Some(url.clone());
                // TODO: async doesnt do anything here, just a remnant from when using tokio, which was incompatible with serde
                return cosmic::task::future(async move {
                    let path = match url.scheme() {
                        "file" => url.to_file_path().unwrap(),
                        other => {
                            return Message::Error(format!("{url} has unknown scheme: {other}"));
                        }
                    };

                    let mut file = match std::fs::File::open(&path) {
                        Ok(file) => file,
                        Err(why) => {
                            return Message::Error(format!(
                                "failed to open {}: {why}",
                                path.display()
                            ));
                        }
                    };

                    let json = match serde_json::from_reader(&mut file) {
                        Ok(json) => json,
                        Err(why) => return Message::Error(format!("Failed to parse JSON: {why}")),
                    };

                    Message::LoadJSON(json)
                });
            },
            Message::Save(url) => {
                // TODO: do this properly without the dummy message
                let message = || {
                    let path = match url.scheme() {
                        "file" => url.to_file_path().unwrap(),
                        other => {
                            return Message::Error(format!("{url} has unknown scheme: {other}"));
                        }
                    };

                    let mut file = match std::fs::File::create(&path) {
                        Ok(file) => file,
                        Err(why) => {
                            return Message::Error(format!(
                                "failed to create {}: {why}",
                                path.display()
                            ));
                        }
                    };

                    if let Err(why) = serde_json::to_writer(&mut file, &self.json) {
                        return Message::Error(format!("Failed to parse JSON: {why}"));
                    };
                    
                    Message::Dummy
                };
                return self.update(message());
            },
            Message::InitJSON(lump) => {
                self.skydefs_index = SkydefsIndex::None;
                match lump {
                    LumpType::GAMECONF => self.json.data = ID24JsonData::gameconf(),
                    LumpType::SKYDEFS => self.json.data = ID24JsonData::skydefs(),
                    _ => ()
                }
            },
            Message::LoadJSON(json) => {
                self.skydefs_index = SkydefsIndex::None;
                self.json = *json;
                // TODO: figure out a nicer way to do this
                self.nav.activate(*self.nav_ids.get(&(&self.json.data).into()).unwrap());
            },
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
                if let ID24JsonData::SKYDEFS { skies, .. } = &mut self.json.data {
                    skies.get_or_insert_with(Vec::new).push(skydefs::Sky::default());
                }
            },
            Message::NewFlatmapping => {
                if let ID24JsonData::SKYDEFS { flatmapping, .. } = &mut self.json.data {
                    flatmapping.get_or_insert_with(Vec::new).push(skydefs::FlatMapping::default());
                }
            },
            Message::DeleteSky(idx) => {
                // TODO: make this async in case the list is very large
                if let ID24JsonData::SKYDEFS { skies: Some(skies), .. } = &mut self.json.data {
                    self.skydefs_index = SkydefsIndex::None;
                    skies.remove(idx);
                }
            },
            Message::DeleteFlatmapping(idx) => {
                if let ID24JsonData::SKYDEFS { flatmapping: Some(flatmapping), .. } = &mut self.json.data {
                    self.skydefs_index = SkydefsIndex::None;
                    flatmapping.remove(idx);
                }
            },
            Message::UpdateSkyTexProp(skymessage) => {
                if let (ID24JsonData::SKYDEFS { skies: Some(skies), .. }, SkydefsIndex::Sky(idx)) = (&mut self.json.data, self.skydefs_index) {
                    match skymessage {
                        SkyTexMessage::ChangeName(name) => skies[idx].name = name,
                        SkyTexMessage::ChangeMid(mid) => skies[idx].mid = mid,
                        SkyTexMessage::ChangeScaleX(scale) => skies[idx].scalex = scale,
                        SkyTexMessage::ChangeScaleY(scale) => skies[idx].scaley = scale,
                        SkyTexMessage::ChangeScrollX(scroll) => skies[idx].scrollx = scroll,
                        SkyTexMessage::ChangeScrollY(scroll) => skies[idx].scrolly = scroll,
                    }
                }
            },
            Message::UpdateSkyTexPropFG(skymessage) => {
                if let (ID24JsonData::SKYDEFS { skies: Some(skies), .. }, SkydefsIndex::Sky(idx)) = (&mut self.json.data, self.skydefs_index) {
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
                if let (ID24JsonData::SKYDEFS { skies: Some(skies), .. }, SkydefsIndex::Sky(idx)) = (&mut self.json.data, self.skydefs_index) {
                    if let Some(fire) = &mut skies[idx].fire {
                        fire.updatetime = speed;
                    }
                }
            },
            Message::ChangeSkyType(sky_type) => {
                if let (ID24JsonData::SKYDEFS { skies: Some(skies), .. }, SkydefsIndex::Sky(idx)) = (&mut self.json.data, self.skydefs_index) {
                    let sky = &mut skies[idx];
                    sky.sky_type = sky_type;
                    match sky_type {
                        skydefs::SkyType::Standard => {
                            sky.foregroundtex = None;
                            sky.fire = None;
                        },
                        skydefs::SkyType::WithForeground => {
                            sky.foregroundtex = Some(skydefs::ForegroundTex::default());
                            sky.fire = None;
                        },
                        skydefs::SkyType::Fire => {
                            sky.foregroundtex = None;
                            sky.fire = Some(skydefs::Fire::default());
                        }
                    }
                }
            },
            Message::EditText(action) => {
                self.text_content.perform(action);
            },
            Message::CloseToast(id) => {
                self.toasts.remove(id);
            },
            Message::Error(e) => {
                self.error_status = Some(e);
            },
            Message::CloseError => {
                self.error_status = None;
            },
            Message::Quit => std::process::exit(0),
            _ => ()
        }

        Task::none()
    }

    #[allow(clippy::too_many_lines)]
    // not sure how much splitting this would help with organization,
    // at least right now, so make it stop yelling at me
    fn view(&self) -> Element<Self::Message> {
        fn aligned_row<'a, Message: 'a>(
            label: &'a str,
            widget: impl Into<Element<'a, Message>>,
        ) -> widget::Row<'a, Message> {
            widget::row()
                .push(widget::text::heading(label))
                .push(widget::horizontal_space())
                .push(widget.into())
                .align_y(Alignment::Center)
        }
        let mut content = Vec::new();
        if let Some(e) = &self.error_status {
            content.push(widget::warning(e).on_close(Message::CloseError).into());
        }
        let main_content: Element<Self::Message> = match self.nav.active_data() {
            Some(LumpType::GAMECONF) => {
                if let ID24JsonData::GAMECONF {
                    title, author, version,
                    iwad, pwadfiles, dehfiles,
                    executable, mode, options,
                    playertranslations, wadtranslation, ..
                } = &self.json.data {
                    let title_input = widget::text_input(
                        "my cool wad",
                        title.as_deref().unwrap_or(""))
                        .on_input(|s| Message::Dummy);
                    let author_input = widget::text_input(
                        "electricbrass",
                        author.as_deref().unwrap_or(""))
                        .on_input(|s| Message::Dummy);
                    let version_input = widget::text_input(
                        "1.0",
                        version.as_deref().unwrap_or(""))
                        .on_input(|s| Message::Dummy);
                    let description_input = widget::text_editor(&self.text_content)
                        .placeholder("a really awesome set of levels")
                        .on_action(Message::EditText);
                    let exe_pick = cosmic::iced::widget::pick_list(
                        id24json::gameconf::Executable::VARIANTS,
                        executable.as_ref(),
                        |e| Message::Dummy
                    ).placeholder("None");
                    let mode_pick = cosmic::iced::widget::pick_list(
                        id24json::gameconf::Mode::VARIANTS,
                        mode.as_ref(),
                        |m| Message::Dummy
                    ).placeholder("None");

                    let list = widget::list_column()
                        .add(aligned_row("Title:", title_input))
                        .add(aligned_row("Author:", author_input))
                        .add(aligned_row("Version:", version_input))
                        .add(aligned_row("Description:", description_input))
                        .add(aligned_row("Executable:", exe_pick))
                        .add(aligned_row("Mode:", mode_pick));

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
            },
            Some(LumpType::SKYDEFS) => {
                if let ID24JsonData::SKYDEFS { skies, flatmapping } = &self.json.data {
                    use skydefs::{Sky, SkyType, ForegroundTex, Fire};
                    let mut properties_list = Vec::new();
                    if let (Some(skies), SkydefsIndex::Sky(idx)) = (skies, self.skydefs_index) {
                        // TODO: use .get and check that the sky exists
                        // we shouldn't run into a case where the index is out of bounds but just in case
                        let Sky {
                            name,
                            mid,
                            scrollx,
                            scrolly,
                            scalex,
                            scaley,
                            sky_type,
                            fire,
                            foregroundtex
                        } = &skies[idx];
                        let name_input = widget::text_input("SKY1", name)
                            .on_input(|s| Message::UpdateSkyTexProp(SkyTexMessage::ChangeName(s)));
                        let mid_spin = widget::spin_button(
                            (*mid).to_string(), *mid,
                            1, 0, 1024, // TODO: figure out proper values for these
                            |v| Message::UpdateSkyTexProp(SkyTexMessage::ChangeMid(v))
                        );
                        let scrollx_spin = widget::spin_button(
                            (*scrollx).to_string(), *scrollx,
                            0.1, 0.0, 100.0,
                            |v| Message::UpdateSkyTexProp(SkyTexMessage::ChangeScrollX(v))
                        );
                        let scrolly_spin = widget::spin_button(
                            (*scrolly).to_string(), *scrolly,
                            0.1, 0.0, 100.0,
                            |v| Message::UpdateSkyTexProp(SkyTexMessage::ChangeScrollY(v))
                        );
                        let scalex_spin = widget::spin_button(
                            (*scalex).to_string(), *scalex,
                            0.1, 0.0, 100.0,
                            |v| Message::UpdateSkyTexProp(SkyTexMessage::ChangeScaleX(v))
                        );
                        let scaley_spin = widget::spin_button(
                            (*scaley).to_string(), *scaley,
                            0.1, 0.0, 100.0,
                            |v| Message::UpdateSkyTexProp(SkyTexMessage::ChangeScaleY(v))
                        );
                        let type_pick = cosmic::iced::widget::pick_list(
                            SkyType::VARIANTS,
                            Some(sky_type),
                            Message::ChangeSkyType
                        );
                        properties_list.push(aligned_row("Texture:", name_input));
                        properties_list.push(aligned_row("Mid:", mid_spin));
                        properties_list.push(aligned_row("Scroll X (seconds):", scrollx_spin));
                        properties_list.push(aligned_row("Scroll Y (seconds):", scrolly_spin));
                        properties_list.push(aligned_row("Scale X:", scalex_spin));
                        properties_list.push(aligned_row("Scale Y:", scaley_spin));
                        properties_list.push(aligned_row("Type:", type_pick));
                        match (sky_type, fire, foregroundtex) {
                            (SkyType::WithForeground, _, Some(ForegroundTex {
                                name,
                                mid,
                                scrollx,
                                scrolly,
                                scalex,
                                scaley })) => {
                                // TODO: reduce duplication in this section and the one above
                                let name_input = widget::text_input("SKY2", name)
                                    .on_input(|s| Message::UpdateSkyTexPropFG(SkyTexMessage::ChangeName(s)));
                                let mid_spin = widget::spin_button(
                                    (*mid).to_string(), *mid,
                                    1, 0, 1024, // TODO: figure out proper values for these
                                    |v| Message::UpdateSkyTexPropFG(SkyTexMessage::ChangeMid(v))
                                );
                                let scrollx_spin = widget::spin_button(
                                    (*scrollx).to_string(), *scrollx,
                                    0.1, 0.0, 100.0,
                                    |v| Message::UpdateSkyTexPropFG(SkyTexMessage::ChangeScrollX(v))
                                );
                                let scrolly_spin = widget::spin_button(
                                    (*scrolly).to_string(), *scrolly,
                                    0.1, 0.0, 100.0,
                                    |v| Message::UpdateSkyTexPropFG(SkyTexMessage::ChangeScrollY(v))
                                );
                                let scalex_spin = widget::spin_button(
                                    (*scalex).to_string(), *scalex,
                                    0.1, 0.0, 100.0,
                                    |v| Message::UpdateSkyTexPropFG(SkyTexMessage::ChangeScaleX(v))
                                );
                                let scaley_spin = widget::spin_button(
                                    (*scaley).to_string(), *scaley,
                                    0.1, 0.0, 100.0,
                                    |v| Message::UpdateSkyTexPropFG(SkyTexMessage::ChangeScaleY(v))
                                );
                                properties_list.push(aligned_row("Foreground Texture:", name_input));
                                properties_list.push(aligned_row("Foreground Mid:", mid_spin));
                                properties_list.push(aligned_row("Foreground Scroll X (seconds):", scrollx_spin));
                                properties_list.push(aligned_row("Foreground Scroll Y (seconds):", scrolly_spin));
                                properties_list.push(aligned_row("Foreground Scale X:", scalex_spin));
                                properties_list.push(aligned_row("Foreground Scale Y:", scaley_spin));
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
                                acc.add(widget::button::text(&sky.name).on_press(
                                    Message::SelectSky(Some(idx))
                                ))
                    ));
                    let flatmapping_list = flatmapping.as_ref().map_or(
                        widget::list_column(),
                        |s| s.iter().enumerate().fold(
                            widget::list_column(),
                            |acc, (idx, mapping)|
                                acc.add(widget::button::text(&mapping.flat).on_press(
                                    Message::SelectFlatmapping(Some(idx))
                                ))
                        ));
                    let content = widget::row::with_children(vec![
                        widget::container(properties_list)
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
            },
            _ => {
                widget::container(widget::text::title3("Unimplemented!!"))
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .into()
            },
        };

        content.push(main_content);

        widget::toaster(&self.toasts, widget::column::with_children(content))
    }
}