#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide the console window on Windows in release

mod skydefs;

use std::fmt::{Display, Formatter};
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Fire {
    updatetime: f32,
    palette: Vec<u8>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ForegroundTex {
    name: String,
    mid: u32,
    scrollx: f32,
    scrolly: f32,
    scalex: f32,
    scaley: f32,
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
enum SkyType {
    Standard = 0,
    Fire = 1,
    WithForeground = 2
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Sky {
    #[serde(rename = "type")]
    sky_type: SkyType,
    name: String,
    mid: u32,
    scrollx: f32,
    scrolly: f32,
    scalex: f32,
    scaley: f32,
    fire: Option<Fire>,
    foregroundtex: Option<ForegroundTex>
}

impl Default for Sky {
    fn default() -> Self {
        Self {
            sky_type: SkyType::Standard,
            name: "SKY1".to_owned(),
            mid: 100,
            scrollx: 0.0,
            scrolly: 0.0,
            scalex: 1.0,
            scaley: 1.0,
            fire: None,
            foregroundtex: None
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct FlatMapping {
    flat: String,
    sky: String
}

impl Default for FlatMapping {
    fn default() -> Self {
        Self {
            flat: "F_SKY1".to_owned(),
            sky: "SKY1".to_owned()
        }
    }
}

impl Display for FlatMapping {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.flat.clone())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(tag = "type", content = "data", rename_all = "lowercase")]
enum ID24JsonData {
    GAMECONF,
    DEMOLOOP,
    SBARDEF,
    SKYDEFS {
        skies: Option<Vec<Sky>>,
        flatmapping: Option<Vec<FlatMapping>>
    },
    TRAKINFO,
    Interlevel,
    Finale
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ID24JsonMetaData {}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ID24Json {
    version: ID24JsonVersion,
    metadata: ID24JsonMetaData,
    #[serde(flatten)]
    data: ID24JsonData,
}

#[derive(Debug)]
struct ID24JsonVersion {
    major: u8,
    minor: u8,
    revision: u8,
}

impl serde::Serialize for ID24JsonVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(format!("{}.{}.{}", self.major, self.minor, self.revision).as_ref())
    }
}

impl<'a> serde::Deserialize<'a> for ID24JsonVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'a> {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(serde::de::Error::custom("Expected format 'major.minor.revision'"));
        }

        let major = parts[0].parse::<u8>()
            .map_err(|_| serde::de::Error::custom("Invalid major version"))?;
        let minor = parts[1].parse::<u8>()
            .map_err(|_| serde::de::Error::custom("Invalid minor version"))?;
        let revision = parts[2].parse::<u8>()
            .map_err(|_| serde::de::Error::custom("Invalid revision version"))?;

        Ok(ID24JsonVersion { major, minor, revision })
    }
}

impl Default for ID24Json {
    fn default() -> Self {
        Self {
            version: ID24JsonVersion { major: 1, minor: 0, revision: 0 },
            metadata: ID24JsonMetaData {},
            data: ID24JsonData::SKYDEFS {
                skies: None,
                flatmapping: None
            }
        }
    }
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
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
            })
        });
    }
}

impl Display for Sky {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.clone())
    }
}

fn list<T: Default + Display>(ui: &mut egui::Ui, heading: &str, list: &mut Vec<T>, list_index: &mut Option<usize>) {
    ui.horizontal(|ui| {
        ui.heading(heading);
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
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.top_menu_bar(ctx);
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
        egui::SidePanel::right("right panel").min_width(100.0).show(ctx, |ui| {
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