pub struct AddItemFields {
    pub name: String,
    pub description: String,
    pub price: String,
}

impl AddItemFields {
    // pub fn new(name: String, description: String, price: f32) -> AddItemFields {
    //     AddItemFields {
    //         name: (name),
    //         description: (description),
    //         price: (price),
    //     }
    // }
    pub fn new() -> AddItemFields {
        AddItemFields {
            name: (String::new()),
            description: (String::new()),
            price: (String::new()),
        }
    }
}
