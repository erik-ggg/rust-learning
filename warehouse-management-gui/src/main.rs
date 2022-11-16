use std::{collections::HashMap, hash::Hash, thread};

use chrono::Utc;

use eframe::egui::{Context, Ui};
use order::{Item, ItemsOperations, Order};
use ui_data::AddItemFields;
use uuid::Uuid;

use ::eframe::egui;

mod order;

mod ui_data;

const ADD_ITEM_WINDOW_HANDLER: &str = "ADD_ITEM_WINDOW_HANDLER";

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp<'a> {
    name: String,
    age: u32,
    orders: Vec<Order>,
    items: Vec<Item>,
    windows_handlers: HashMap<&'a str, bool>,
    add_item_fields: AddItemFields,
}

impl Default for MyApp<'_> {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
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
                if ui.button("Show all items").clicked() {
                    for item in self.items.iter() {
                        item.show();
                    }
                }

                if ui.button("Add new item").clicked() {
                    self.windows_handlers.insert(ADD_ITEM_WINDOW_HANDLER, true);
                }
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
            app.items.push(Item::new(
                name.clone(),
                description.clone(),
                price.clone().parse().unwrap(),
            ));
            app.windows_handlers.insert(ADD_ITEM_WINDOW_HANDLER, false);
        };
        if ui.button("Close").clicked() {
            app.windows_handlers.insert(ADD_ITEM_WINDOW_HANDLER, false);
        };
    });
}

// fn add_item_panel() -> {

// }

// let mut orders = Vec::<Order>::new();
// let mut items = Vec::<Item>::new();

// let mut order = Order {
//     id: Uuid::new_v4(),
//     items: Vec::new(),
//     creation_date: Utc::now(),
// };
// order.show();

// let item = Item {
//     id: Uuid::new_v4(),
//     name: String::from("Television"),
//     description: String::from("Awesome 32K TV"),
//     price: 9999.99,
// };

// items.push(item.clone());

// order.items.push(item);
// order.show();

// // let mut items = items.clone();
// // let mut orders = orders.clone();

// loop {
//     let stdin = std::io::stdin();
//     let mut line_buf = String::new();
//     match stdin.read_line(&mut line_buf) {
//         Ok(fc) => fc,
//         Err(_) => break,
//     };

//     // let mut items = items.clone();
//     items.push(Item::new(line_buf.trim_end().to_string()));

//     // let mut orders = orders.clone();

//     let order = Order::new(items.clone(), Utc::now());
//     orders.push(order);
//     for order in orders.iter() {
//         order.show();
//     }
//     println!("\n\n\n\n")
// }
// }
