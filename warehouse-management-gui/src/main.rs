#[macro_use]
extern crate log;

use std::{collections::HashMap, hash::Hash, thread};

use chrono::Utc;

use eframe::egui::{Context, Ui};
use egui_extras::{Size, TableBuilder};
use order::{Item, ItemsOperations, Order};
use ui_data::AddItemFields;
use uuid::Uuid;

use ::eframe::egui;

mod order;

mod ui_data;

const ADD_ITEM_WINDOW_HANDLER: &str = "ADD_ITEM_WINDOW_HANDLER";

fn main() {
    env_logger::init();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp<'a> {
    orders: Vec<Order>,
    items: Vec<Item>,
    windows_handlers: HashMap<&'a str, bool>,
    add_item_fields: AddItemFields,
}

impl Default for MyApp<'_> {
    fn default() -> Self {
        Self {
            orders: Vec::new(),
            items: Vec::new(),
            windows_handlers: set_windows_handlers_hashmap(),
            add_item_fields: AddItemFields::new(),
        }
    }
}

fn set_windows_handlers_hashmap() -> HashMap<&'static str, bool> {
    let mut handlers = HashMap::new();

    handlers.insert(ADD_ITEM_WINDOW_HANDLER, false);

    handlers
}

impl eframe::App for MyApp<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::collapsing_header::CollapsingState::load_with_default_open(
                ui.ctx(),
                ui.make_persistent_id("order_collapsing_header"),
                false,
            )
            .show_header(ui, |ui| {
                ui.heading("Order Manager");
            })
            .body(|ui| ui.label("Body"));

            egui::collapsing_header::CollapsingState::load_with_default_open(
                ui.ctx(),
                ui.make_persistent_id("item_collapsing_header"),
                false,
            )
            .show_header(ui, |ui| {
                ui.heading("Item Manager");
            })
            .body(|ui| {
                if ui.button("Add new item").clicked() {
                    self.windows_handlers.insert(ADD_ITEM_WINDOW_HANDLER, true);
                }

                items_table_content(ui, &mut self.items);
            });

            if self
                .windows_handlers
                .get(ADD_ITEM_WINDOW_HANDLER)
                .unwrap()
                .to_owned()
            {
                egui::Window::new("Add Item")
                    .open(&mut true)
                    .resizable(false)
                    .show(ctx, |ui| add_item_window_content(self, ui));
            }
        });
    }
}

fn add_item_window_content(app: &mut MyApp, ui: &mut Ui) {
    egui::Grid::new("add_item_window_grid")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label("Name: ");
            ui.add(egui::TextEdit::singleline(&mut app.add_item_fields.name));
            ui.end_row();

            ui.label("Description: ");
            ui.add(egui::TextEdit::singleline(
                &mut app.add_item_fields.description,
            ));
            ui.end_row();

            ui.label("Price: ");
            ui.add(egui::TextEdit::singleline(&mut app.add_item_fields.price));
            ui.end_row();
        });

    ui.separator();

    ui.horizontal(|ui| {
        let name = &app.add_item_fields.name;
        let description = &app.add_item_fields.description;
        let price = &app.add_item_fields.price;

        if ui.button("Add").clicked() {
            let result = match price.clone().parse::<f32>() {
                Ok(value) => value,
                Err(err) => {
                    error!("{err}");
                    -1.0
                }
            };

            if result != -1.0 {
                app.items.push(Item::new(
                    name.clone(),
                    description.clone(),
                    match price.clone().parse() {
                        Ok(value) => value,
                        Err(err) => {
                            error!("{err}");
                            -1.0
                        }
                    },
                ));

                app.add_item_fields = AddItemFields::new();
                app.windows_handlers.insert(ADD_ITEM_WINDOW_HANDLER, false);
            } else {
                // show error
            }
        };
        if ui.button("Close").clicked() {
            app.windows_handlers.insert(ADD_ITEM_WINDOW_HANDLER, false);
        };
    });
}

fn items_table_content(ui: &mut Ui, items: &mut Vec<Item>) {
    let table = TableBuilder::new(ui)
        .striped(true)
        .column(Size::initial(100.0).at_least(40.0))
        .column(Size::initial(60.0).at_least(40.0))
        .column(Size::initial(500.0).at_least(40.0));

    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.heading("Name");
            });
            header.col(|ui| {
                ui.heading("Price");
            });
            header.col(|ui| {
                ui.heading("Description");
            });
        })
        .body(|mut body| {
            for item in items {
                body.row(18.0, |mut row| {
                    row.col(|ui| {
                        ui.label(&item.name);
                    });
                    row.col(|ui| {
                        ui.label(&item.price.to_string());
                    });
                    row.col(|ui| {
                        ui.label(&item.description);
                    });
                });
            }
        });
}
