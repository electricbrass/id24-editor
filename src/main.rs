#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide the console window on Windows in release

mod id24json;

use id24json::{ID24Json, ID24JsonData};
use id24json::skydefs::{SkyType};

use std::fmt::Display;
use std::collections::HashMap;
use cosmic::widget::menu;
use cosmic::widget::menu::key_bind::{KeyBind, Modifier};
use cosmic::{iced::keyboard::Key, iced_core::keyboard::key::Named};
use eframe::egui;
use egui_extras::{Column, TableBuilder};

use cosmic::prelude::*;

// TODO: before making too much gui progress, decide if egui is the right option
// iced or fltk-rs might be a better option for a retained-mode gui
fn main() -> cosmic::iced::Result {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
    //     ..Default::default()
    // };
    // eframe::run_native(
    //     "ID24 JSON Editor",
    //     options,
    //     Box::new(|cc| {
    //         Ok(Box::<MyApp>::default())
    //     }),
    // )
    let settings = cosmic::app::Settings::default();
    cosmic::app::run::<MyCosmicApp>(settings, ())
}

#[derive(Debug, PartialEq)]
enum LumpType {
    GAMECONF,
    DEMOLOOP,
    SBARDEF,
    SKYDEFS,
    TRAKINFO,
    Interlevel,
    Finale,
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
            MyMenuAction::Open => Message::Open,
            MyMenuAction::Save => Message::Save,
            MyMenuAction::SaveAs => Message::SaveAs,
            MyMenuAction::Quit => Message::Quit,
        }
    }
}


struct MyCosmicApp {
    core: cosmic::Core,
    key_binds: HashMap<KeyBind, MyMenuAction>,
    counter: u32,
    counter_text: String,
    json: ID24Json,
}

#[derive(Debug, Clone)]
enum Message {
    Clicked,
    Open,
    Save,
    SaveAs,
    Quit,
    Error(String),
    Dummy
}

impl cosmic::Application for MyCosmicApp {
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
        let mut app = MyCosmicApp {
            core,
            key_binds: HashMap::from([
                // TODO: figure out why keybinds dont work and fix display of Ctrl+Shift+S being cut off
                (KeyBind { modifiers: vec![Modifier::Ctrl], key: Key::Character("o".into()) }, MyMenuAction::Open),
                (KeyBind { modifiers: vec![Modifier::Ctrl], key: Key::Character("s".into()) }, MyMenuAction::Save),
                (KeyBind { modifiers: vec![Modifier::Ctrl, Modifier::Shift], key: Key::Character("s".into()) }, MyMenuAction::SaveAs),
                (KeyBind { modifiers: vec![Modifier::Ctrl], key: Key::Character("q".into()) }, MyMenuAction::Quit),
            ]),
            counter: 0,
            counter_text: "this is a counter".to_owned(),
            json: ID24Json::default()
        };
        let command = app.set_window_title("ID24 JSON Editor".to_owned());
        (app, command)
    }

    fn view(&self) -> Element<Self::Message> {
        match &self.json.data {
            ID24JsonData::SKYDEFS { skies, flatmapping } => {
                let button = cosmic::widget::button::standard(&self.counter_text)
                    .on_press(Message::Clicked);

                cosmic::widget::container(button)
                    .center_x(cosmic::iced::Length::Fill)
                    .center_y(cosmic::iced::Length::Shrink)
                    .into()
            },
            _ => todo!()
        }
    }

    fn update(&mut self, message: Self::Message) -> cosmic::app::Task<Self::Message> {
        match message {
            Message::Clicked => {
                self.counter += 1;
                self.counter_text = format!("Clicked {} times", self.counter);
            },
            Message::Open => {
                return cosmic::task::future(async {
                    use cosmic::dialog::file_chooser;
                    let filter = file_chooser::FileFilter::new("JSON Files").extension("json");
                    let dialog = file_chooser::open::Dialog::new()
                        .filter(filter);
                    // TODO: should i open the file here directly or add another message for that? like in the example
                    match dialog.open_file().await {
                        Ok(response) => { println!("selected to open {:?}", response.url()); Message::Dummy },
                        // TODO: probably make a message that just logs smth to stderr
                        Err(file_chooser::Error::Cancelled) => Message::Dummy,
                        // TODO: display this error somehow, is a string the best way to store it?
                        Err(why) => Message::Error(why.to_string()),
                    }
                });
            },
            Message::Quit => std::process::exit(0),
            _ => ()
        }

        Task::none()
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
}

struct MyApp {
    json: ID24Json,
    current_editor: LumpType,
    selected_sky_index: Option<usize>,
    selected_flatmap_index: Option<usize>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            json: ID24Json::default(),
            current_editor: LumpType::SKYDEFS,
            selected_sky_index: None,
            selected_flatmap_index: None
        }
    }
}

impl MyApp {
    fn top_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    // TODO: add some real error handling instead of expects
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("JSON", &["json"])
                            .pick_file() {
                            let file = std::fs::File::open(path)
                                .expect("file should open read only");
                            self.json = serde_json::from_reader(file).expect("file should be valid id24 json");
                            println!("json: {:?}", self.json);
                        }
                    }
                    if ui.button("Save As").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("JSON", &["json"])
                            .save_file() {
                            let file = std::fs::File::create(path)
                                .expect("file should open write only");
                            serde_json::to_writer_pretty(file, &self.json).expect("json should have been written i hope");
                        }
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
            })
        });
    }
}

// TODO: name this better
fn list<T: Default + Display>(ui: &mut egui::Ui, heading: &str, list: &mut Vec<T>, list_index: &mut Option<usize>) {
    ui.push_id(heading, |ui| {  // Add unique ID scope
        ui.heading(heading);
        ui.horizontal(|ui| {
            if ui.button("➕").clicked() {
                list.push(T::default());
                *list_index = Some(list.len() - 1);
            }
            if ui.button("❌").clicked() {
                if let Some(index) = list_index {
                    list.remove(*index);
                    *list_index = None;
                }
            }
        });

        // TODO: figure out if TableBuilder is even worth it here
        TableBuilder::new(ui)
            .column(Column::auto())  // For the item name
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Items");
                });
            })
            .body(|mut body| {
                for (index, item) in list.iter().enumerate() {
                    body.row(20.0, |mut row| {
                        row.col(|ui| {
                            let is_selected = *list_index == Some(index);
                            if ui.selectable_label(is_selected, item.to_string()).clicked() {
                                *list_index = Some(index);
                            }
                        });
                    });
                }
            });
    });
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.top_menu_bar(ctx);
        // TODO: maybe use egui_dock for proper tabs?
        egui::TopBottomPanel::top("editor_tabs").show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.selectable_value(&mut self.current_editor, LumpType::GAMECONF, "GAMECONF");
                ui.selectable_value(&mut self.current_editor, LumpType::DEMOLOOP, "DEMOLOOP");
                ui.selectable_value(&mut self.current_editor, LumpType::SBARDEF, "SBARDEF");
                ui.selectable_value(&mut self.current_editor, LumpType::SKYDEFS, "SKYDEFS");
                ui.selectable_value(&mut self.current_editor, LumpType::TRAKINFO, "TRAKINFO");
                ui.selectable_value(&mut self.current_editor, LumpType::Interlevel, "Interlevel");
                ui.selectable_value(&mut self.current_editor, LumpType::Finale, "Finale");
            });
        });
        // TODO: move this and the central panel inside the match so that they're not separately checking for skydefs
        egui::SidePanel::right("right panel").min_width(75.0).show(ctx, |ui| {
            // TODO: make it so that the lists dont move and resize as elements are added and removed, we do still want them resizable by users
            if let ID24JsonData::SKYDEFS { skies, flatmapping } = &mut self.json.data {
                if skies.is_none() {
                    *skies = Some(Vec::new());
                }
                if let Some(skies) = skies {
                    list(ui, "Skies", skies, &mut self.selected_sky_index);
                }
                ui.separator();
                if flatmapping.is_none() {
                    *flatmapping = Some(Vec::new());
                }
                if let Some(flatmapping) = flatmapping {
                    list(ui, "Flat Mappings", flatmapping, &mut self.selected_flatmap_index);
                }
            }
        });

        // TODO: split match arms into separate functions
        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO: don't hard code everything at the top level to be skydefs
            match &mut self.json.data {
                ID24JsonData::SKYDEFS { skies, .. } => {
                    if let Some(skies) = skies.as_mut() {
                        if let Some(selected_index) = self.selected_sky_index {
                            if let Some(selected_sky) = skies.get_mut(selected_index) {
                                egui::Grid::new("sky_parameters_grid")
                                    .num_columns(2)
                                    .spacing([36.0, 4.0])
                                    .show(ui, |ui| {
                                        ui.label("Name:");
                                        let text_edit = egui::TextEdit::singleline(&mut selected_sky.name)
                                            .desired_width(75.0);
                                        ui.add(text_edit);

                                        ui.end_row();

                                        // TODO: display units for all of these

                                        ui.label("Mid:");
                                        ui.add(egui::DragValue::new(&mut selected_sky.mid).speed(1));
                                        ui.end_row();

                                        ui.label("Scroll X:");
                                        ui.add(egui::DragValue::new(&mut selected_sky.scrollx).speed(0.1));
                                        ui.end_row();

                                        ui.label("Scroll Y:");
                                        ui.add(egui::DragValue::new(&mut selected_sky.scrolly).speed(0.1));
                                        ui.end_row();

                                        ui.label("Scale X:");
                                        ui.add(egui::DragValue::new(&mut selected_sky.scalex).speed(0.1));
                                        ui.end_row();

                                        ui.label("Scale Y:");
                                        ui.add(egui::DragValue::new(&mut selected_sky.scaley).speed(0.1));
                                        ui.end_row();

                                        ui.label("Sky type:");
                                        egui::ComboBox::new("sky_type", "")
                                            .selected_text(format!("{:?}", selected_sky.sky_type))
                                            .show_ui(ui, |ui| {
                                                for kind in [
                                                    SkyType::Standard,
                                                    SkyType::Fire,
                                                    SkyType::WithForeground,
                                                ] {
                                                    ui.selectable_value(&mut selected_sky.sky_type, kind, format!("{:?}", kind));
                                                }
                                            });
                                        ui.end_row();
                                    });
                            }
                        }
                    }
                }
                _ => { ui.label("Unimplemented!"); }
            }
        });
    }
}