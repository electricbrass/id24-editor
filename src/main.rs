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

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide the console window on Windows in release

mod id24json;
mod pages;
mod widgets;
mod config;

use id24json::{ID24Json, ID24JsonData};

use std::fmt::{Display, Formatter};
use std::collections::HashMap;
use cosmic::{widget, Application};
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::widget::{menu, nav_bar};
use cosmic::widget::menu::key_bind::{KeyBind, Modifier};
use cosmic::iced::keyboard::{Key, Modifiers};
use cosmic::iced::{event, keyboard, Length, Subscription};
use cosmic::prelude::*;
use cosmic::widget::menu::{Action, ItemWidth};
use strum::IntoEnumIterator;

// TODO: figure out if a context page is how the different editors should be shown

// TODO: figure out how to bundle icons on windows/mac
// maybe make a pr to libcosmic for that

// TODO: write rust port of MTrop's DoomStruct or something like it, existing options are unmaintained and too unfinished
// focus first on the subset that will allow extracting the assets needed for more graphical editors and verifying that lumps referenced in json exist

// TODO: clean up module structure and imports, dont really want super long qualified names but need to avoid clashes too

// TODO: add WAD/PK3 setting in settings page, in WAD mode force all lump fields to 8 characters max and uppercase
// make it persist between sessions, but override it if the user loads from a WAD/PK3 (currently loading from json files directly is all that's supported)

struct Flags {
    config: config::Config,
    config_handler: Option<cosmic_config::Config>
}

// TODO: add cli options for loading files and maybe doing a couple other things
fn main() -> cosmic::iced::Result {
    let settings = cosmic::app::Settings::default();
    let (config, config_handler) = match cosmic_config::Config::new(EditorModel::APP_ID, config::Config::VERSION) {
        Ok(config_handler) => {
            let config = config::Config::get_entry(&config_handler).unwrap_or_else(|(errs, config)| {
                println!("errors loading config: {errs:?}");
                config
            });
            (config, Some(config_handler))
        },
        Err(err) => {
            println!("failed to create config handler: {err}");
            (config::Config::default(), None)
        }
    };
    let flags = Flags { config, config_handler };
    cosmic::app::run::<EditorModel>(settings, flags)
}

// TODO: consider using std::mem::discriminant instead of this
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
    toasts: widget::Toasts<Message>,
    config: config::Config,
    config_handler: Option<cosmic_config::Config>,
    error_status: Option<String>,
    current_file: Option<url::Url>,
    json: ID24Json,
    // TODO: should these be optional and be None when not active?
    skydefs_page: pages::skydefs::Page,
    gameconf_page: pages::gameconf::Page,
    demoloop_page: pages::demoloop::Page,
}

#[derive(Debug, Clone)]
enum Message {
    // TODO: split each editor into its own module with its own message type
    GameconfMessage(pages::gameconf::Message),
    SkydefsMessage(pages::skydefs::Message),
    DemoloopMessage(pages::demoloop::Message),
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
    Key(Modifiers, Key),
    Dummy
}

impl From<pages::skydefs::Message> for Message {
    fn from(message: pages::skydefs::Message) -> Self {
        Message::SkydefsMessage(message)
    }
}

impl From<pages::gameconf::Message> for Message {
    fn from(message: pages::gameconf::Message) -> Self {
        Message::GameconfMessage(message)
    }
}

impl From<pages::demoloop::Message> for Message {
    fn from(message: pages::demoloop::Message) -> Self {
        Message::DemoloopMessage(message)
    }
}

fn convert_action_message<M, N: From<M>>(action: cosmic::Action<M>) -> cosmic::Action<N> {
    match action {
        cosmic::Action::None => cosmic::Action::None,
        cosmic::Action::App(m) => cosmic::Action::App(m.into()),
        cosmic::Action::Cosmic(a) => cosmic::Action::Cosmic(a),
    }
}

impl cosmic::Application for EditorModel {
    type Executor = cosmic::executor::Default;
    type Flags = Flags;
    type Message = Message;
    const APP_ID: &'static str = "io.github.electricbrass.id24-editor";

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
                (KeyBind { modifiers: vec![Modifier::Ctrl], key: Key::Character("o".into()) }, MyMenuAction::Open),
                (KeyBind { modifiers: vec![Modifier::Ctrl], key: Key::Character("s".into()) }, MyMenuAction::Save),
                (KeyBind { modifiers: vec![Modifier::Ctrl, Modifier::Shift], key: Key::Character("s".into()) }, MyMenuAction::SaveAs),
                (KeyBind { modifiers: vec![Modifier::Ctrl], key: Key::Character("q".into()) }, MyMenuAction::Quit),
            ]),
            toasts: widget::Toasts::new(Message::CloseToast),
            config: flags.config,
            config_handler: flags.config_handler,
            error_status: None,
            current_file: None,
            json: ID24Json::default(),
            gameconf_page: pages::gameconf::Page::default(),
            skydefs_page: pages::skydefs::Page::default(),
            demoloop_page: pages::demoloop::Page::default(),
        };
        app.set_header_title("ID24 JSON Editor".to_owned());
        let command = app.set_window_title("ID24 JSON Editor".to_owned());
        (app, command)
    }

    fn header_start(&self) -> Vec<Element<'_, Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            widget::RcElementWrapper::new(Element::from(
                menu::root("File"),
            )),
            menu::items(
                &self.key_binds,
                vec![
                    menu::Item::Button("Open", None, MyMenuAction::Open),
                    menu::Item::Button("Save", None, MyMenuAction::Save),
                    menu::Item::Button("Save As", None, MyMenuAction::SaveAs),
                    menu::Item::Button("Quit", None, MyMenuAction::Quit)
                ],
            ),
        )]).item_width(ItemWidth::Uniform(200));

        vec![menu_bar.into()]
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    // TODO: figure out why this is being called when pressing the enter key in the save as dialog
    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<cosmic::Action<Message>> {
        self.nav.activate(id);
        if let Some(lump) = self.nav.data::<LumpType>(id) {
            self.skydefs_page.reset_index();
            return self.update(Message::InitJSON(*lump));
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        event::listen_with(|event, status, _window_id| match event {
            event::Event::Keyboard(keyboard::Event::KeyPressed { modifiers, key, .. }) => {
                match status {
                    event::Status::Ignored => Some(Message::Key(modifiers, key)),
                    event::Status::Captured => None,
                }
            }
            _ => None,
        })
    }

    #[allow(clippy::too_many_lines)]
    // TODO: split this up, just dont want it to yell at me for just a bit longer
    fn update(&mut self, message: Self::Message) -> cosmic::Task<cosmic::Action<Self::Message>> {
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
                // TODO: make this change the current file for saving
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
                // TODO: probably should stop the user from doing invalid things sooner
                if let Err(why) = self.json.data.verify() {
                    // TODO: this should probably be a popup that is required to be dismissed
                    return self.update(Message::Error(format!("Failed to verify JSON: {why}")));
                }
                // TODO: maybe move this into Save As somehow, dont need to be setting it every time we save
                // and/or make a message just for this. would need to figure out how to send multiple messages from Open
                self.current_file = Some(url.clone());
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

                    if let Err(why) = serde_json::to_writer_pretty(&mut file, &self.json) {
                        return Message::Error(format!("Failed to write JSON: {why}"));
                    };

                    Message::Dummy
                };
                return self.update(message());
            },
            Message::InitJSON(lump) => {
                self.skydefs_page.reset_index();
                match lump {
                    LumpType::GAMECONF => self.json.data = ID24JsonData::gameconf(),
                    LumpType::SKYDEFS => self.json.data = ID24JsonData::skydefs(),
                    LumpType::DEMOLOOP => self.json.data = ID24JsonData::demoloop(),
                    _ => ()
                }
            },
            Message::LoadJSON(json) => {
                self.skydefs_page.reset_index();
                self.json = *json;
                // TODO: figure out a nicer way to do this
                self.nav.activate(*self.nav_ids.get(&(&self.json.data).into()).unwrap());
            },
            Message::SkydefsMessage(message) => {
                return self.skydefs_page.update(&mut self.json, message).map(convert_action_message);
            },
            Message::GameconfMessage(message) => {
                return self.gameconf_page.update(&mut self.json, message).map(convert_action_message);
            },
            Message::DemoloopMessage(message) => {
                return self.demoloop_page.update(&mut self.json, message).map(convert_action_message);
            },
            Message::Key(modifiers, key) => {
                for (key_bind, action) in &self.key_binds {
                    if key_bind.matches(modifiers, &key) {
                        return self.update(action.message());
                    }
                }
            },
            Message::CloseToast(id) => self.toasts.remove(id),
            Message::Error(e) => self.error_status = Some(e),
            Message::CloseError => self.error_status = None,
            Message::Quit => std::process::exit(0),
            _ => ()
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let mut content = Vec::new();
        if let Some(e) = &self.error_status {
            content.push(widget::warning(e).on_close(Message::CloseError).into());
        }
        let main_content: Element<Self::Message> = match self.nav.active_data() {
            Some(LumpType::GAMECONF) => {
                self.gameconf_page.view(&self.json).map(Message::GameconfMessage)
            },
            Some(LumpType::SKYDEFS) => {
                self.skydefs_page.view(&self.json).map(Message::SkydefsMessage)
            },
            Some(LumpType::DEMOLOOP) => {
                self.demoloop_page.view(&self.json).map(Message::DemoloopMessage)
            },
            _ => {
                widget::container(widget::text::title3("⇐ Select a lump type"))
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .into()
            },
        };

        content.push(main_content);

        widget::toaster(&self.toasts, widget::column::with_children(content))
    }
}
