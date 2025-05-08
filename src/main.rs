#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide the console window on Windows in release

mod id24json;

use id24json::{ID24Json, ID24JsonData};
use id24json::skydefs::{SkyType};

use std::fmt::Display;
use eframe::egui;
use egui_extras::{Column, TableBuilder};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "ID24 JSON Editor",
        options,
        Box::new(|cc| {
            Ok(Box::<MyApp>::default())
        }),
    )
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