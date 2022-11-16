use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f32,
}

impl Item {
    pub fn new(name: String, description: String, price: f32) -> Item {
        Item {
            id: (Uuid::new_v4()),
            name: (name),
            description: (description),
            price: (price),
        }
    }
}

#[derive(Clone)]
pub struct Order {
    pub id: Uuid,
    pub items: Vec<Item>,
    pub creation_date: DateTime<Utc>,
}

impl Order {
    pub fn new(items: Vec<Item>, creation_date: DateTime<Utc>) -> Order {
        Order {
            id: (Uuid::new_v4()),
            items: (items),
            creation_date: (creation_date),
        }
    }
}

pub trait ItemsOperations {
    fn show(&self);
}

impl ItemsOperations for Item {
    fn show(&self) {
        println!("Item: {}, Price {}", self.name, self.price);
    }
}

impl ItemsOperations for Order {
    fn show(&self) {
        println!(
            "Order ID: {}\nCreated at: {}\n",
            self.id, self.creation_date
        );
        for item in self.items.iter() {
            println!("Item: {}, Price {}", item.name, item.price);
        }
    }
}
