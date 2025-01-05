use iced::widget::{button, column, container, row, text, text_input, checkbox};
use iced::{Element, Length, Settings, Subscription, Theme, Task};
use crate::Receipt;
use crate::items::{Item, create_items_table};
use crate::{Tax, TaxGroup, Gratuity, ServiceCharge, Quantity};
use crate::window::{Window, settings};
use iced::window;
use iced_core;
use std::collections::HashMap;

pub mod custom_appearances;
use custom_appearances::validator;

#[derive(Debug, Clone)]

pub struct ItemView {
    //windows: HashMap<window::Id, Window>,
    items: Vec<Item>,
    edit_states: std::collections::HashMap<i64, bool>,
    
    item_id: validator::Input,
    item_name: String,
    category: String,
    price: validator::Input,
    tax_group: String,
    tax_overide: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    //modify item list and states
    DeleteItem(i64),
    SaveItem(i64),
    NewItem,
    ToggleEditMode(i64, bool),

    //add item form
    ItemIdChanged(validator::Message),
    ItemNameChanged(String),
    CategoryChanged(String),
    PriceChanged(validator::Message),
    TaxGroupChanged(String),
    TaxOverideChanged(bool),
}

impl ItemView {
    pub fn new() -> Self {
        ItemView {
            //windows: HashMap::from([(iced_core::window::Id::MAIN, Window::new("Main Window".to_string()))]),
            items: Vec::new(),
            edit_states: std::collections::HashMap::new(),
            
            item_id: validator::Input::default(),
            item_name: String::new(),
            category: String::new(),
            price: validator::Input::default(),
            tax_group: String::new(),
            tax_overide: false,
        }
    }

    pub fn update(state: &mut Self, message: Message) -> Task<Message> {
        match message {
            //modify item list and states
            Message::DeleteItem(item_id) => {
                state.items.retain(|x| x.id != item_id);
                Task::none()
            }
            Message::SaveItem(item_id) => {
                Task::none()
            }
            Message::NewItem => {
/*                 for item in &state.items {
                    println!("Item: {}", item.name)
                } */

                //create sales tax to add to tax group
                let sales_tax = Tax::new(1, 0.085, "Sales Tax".to_string());

                //add sales tax to a new Vec 'taxes'
                let mut taxes = Vec::new();
                &taxes.push(sales_tax);
                
                //create new taxgroup with the sales tax
                let tax_group = TaxGroup::new(1, "Default".to_string(), taxes);
                
                //create default gratuity
                let gratuity = Gratuity::new(1, 0.20, "Default Gratuity".to_string(), tax_group.clone(), false);
                
                //create default service charge
                let service_charge = ServiceCharge::new(1, 0.05, "Default Service Charge".to_string(), tax_group.clone(), false);

                let id = convert_to_i64(&state.item_id.value);
                let price = convert_to_f64(&state.price.value);

                let new_item = Item::new(id, state.item_name.clone(), state.category.clone(), price, tax_group, state.tax_overide.clone());

                state.items.push(new_item);

                state.item_id = validator::Input::default();
                state.item_name = String::new();
                state.category = String::new();
                state.price = validator::Input::default();
                state.tax_group = String::new();
                state.tax_overide = false;
                
                Task::none()
            }
            Message::ToggleEditMode(item_id, editing) => {
                if editing{
                    state.edit_states.insert(item_id, editing);
                } else {
                    state.edit_states.remove(&item_id);
                }
                Task::none()
            }

            //add item form
            Message::ItemIdChanged(id) => {
                println!("ID Changed");
                match id {
                    validator::Message::RawInput(input) => {
                        state.item_id.value = input;
                        state.item_id.is_valid =
                            validator::validate(&state.item_id.value, validate_i64);
                    }
                    validator::Message::RawSubmit(input) => {
                        state.item_id.value = input;
                        state.item_id.is_valid =
                            validator::validate(&state.item_id.value, validate_i64);

                    }
                }

                Task::none()
            }
            Message::ItemNameChanged(name) => {
                println!("Item Name field Changed");
                state.item_name = name;

                Task::none()
            }
            Message::CategoryChanged(category) => {
                println!("Item Category field Changed");
                state.category = category;

                Task::none()
            }
            Message::PriceChanged(price) => {
                println!("Price Changed");
                match price {
                    validator::Message::RawInput(input) => {
                        state.price.value = input;
                        state.price.is_valid =
                            validator::validate(&state.price.value, validate_f64);
                    }
                    validator::Message::RawSubmit(input) => {
                        state.price.value = input;
                        state.price.is_valid =
                            validator::validate(&state.price.value, validate_f64);

                    }
                }
            
                Task::none()
            }
            Message::TaxGroupChanged(taxgroup) => {
                state.tax_group = taxgroup;

                Task::none()
            }
            Message::TaxOverideChanged(taxoveride) => {
                state.tax_overide = taxoveride;
                Task::none()
            }
        }
    }

    pub fn view<'a>(state: &Self) -> Element<'static, Message>{
        let validator::Input {
            value,
            is_valid,
            placeholder,
        } = &state.item_id;

        let validator::Input {
            value,
            is_valid,
            placeholder,
        } = &state.price;


        //create sales tax to add to tax group
        let sales_tax = Tax::new(1, 0.085, "Sales Tax".to_string());

        //add sales tax to a new Vec 'taxes'
        let mut taxes = Vec::new();
        &taxes.push(sales_tax);

        //create new taxgroup with the sales tax
        let tax_group = TaxGroup::new(1, "Default".to_string(), taxes);

        //create default gratuity
        let gratuity = Gratuity::new(1, 0.20, "Default Gratuity".to_string(), tax_group.clone(), false);

        //create default service charge
        let service_charge = ServiceCharge::new(1, 0.05, "Default Service Charge".to_string(), tax_group.clone(), false);

        column![
            column![

                row![
                    text("Items").size(25),

                ],
                
                row![
                    column![
                        validator::view(&state.item_id.value.clone(), &"Item ID").map(Message::ItemIdChanged),
                        if state.item_id.value.is_empty(){
                            "".into()
                        } else if state.item_id.is_valid {
                            text("Perfect").style(text::primary)
                        } else {
                            text("Numbers Only").style(text::danger)
                        },
                    ],
                    //text_input("Item ID", &state.item_id).on_input(Message::ItemIdChanged),
                    text_input("Item Name", &state.item_name.clone()).on_input(Message::ItemNameChanged),
                    text_input("Item Category", &state.category.clone()).on_input(Message::CategoryChanged),
                    column![
                        validator::view(&state.price.value.clone(), &"Item Price").map(Message::PriceChanged),
                        if state.price.value.is_empty(){
                            "".into()
                        } else if state.price.is_valid {
                            text("Perfect").style(text::primary)
                        } else {
                            text("Numbers Only").style(text::danger)
                        },
                    ],
                    //text_input("Item Price", &state.price).on_input(Message::PriceChanged),
                    text_input("Item Tax Group", &state.tax_group.clone()).on_input(Message::TaxGroupChanged),
                    column![checkbox("Overide Tax", state.tax_overide.clone()).on_toggle(Message::TaxOverideChanged).spacing(2)].spacing(10).padding(10),
                ],
                row![
                    button("Add Item").on_press(Message::NewItem)
                    .width(Length::Shrink),
                ],

                create_items_table(state.items.clone(), &mut state.edit_states.clone())
            ]
        ].into()
    }

    pub fn add_item_window<'a>(state: &Self) -> Element<'static, Message>{
        let validator::Input {
            value,
            is_valid,
            placeholder,
        } = &state.item_id;

        let validator::Input {
            value,
            is_valid,
            placeholder,
        } = &state.price;


        //create sales tax to add to tax group
        let sales_tax = Tax::new(1, 0.085, "Sales Tax".to_string());

        //add sales tax to a new Vec 'taxes'
        let mut taxes = Vec::new();
        &taxes.push(sales_tax);

        //create new taxgroup with the sales tax
        let tax_group = TaxGroup::new(1, "Default".to_string(), taxes);

        //create default gratuity
        let gratuity = Gratuity::new(1, 0.20, "Default Gratuity".to_string(), tax_group.clone(), false);

        //create default service charge
        let service_charge = ServiceCharge::new(1, 0.05, "Default Service Charge".to_string(), tax_group.clone(), false);

        column![
            column![
                row![
                    text("Items").size(25),

                ],
                
                row![
                    column![
                        validator::view(&state.item_id.value.clone(), &"Item ID").map(Message::ItemIdChanged),
                        if state.item_id.value.is_empty(){
                            "".into()
                        } else if state.item_id.is_valid {
                            text("Perfect").style(text::primary)
                        } else {
                            text("Numbers Only").style(text::danger)
                        },
                    ],
                    //text_input("Item ID", &state.item_id).on_input(Message::ItemIdChanged),
                    text_input("Item Name", &state.item_name).on_input(Message::ItemNameChanged),
                    text_input("Item Category", &state.category).on_input(Message::CategoryChanged),
                    column![
                        validator::view(&state.price.value, &"Item Price").map(Message::PriceChanged),
                        if state.price.value.is_empty(){
                            "".into()
                        } else if state.price.is_valid {
                            text("Perfect").style(text::primary)
                        } else {
                            text("Numbers Only").style(text::danger)
                        },
                    ],
                    //text_input("Item Price", &state.price).on_input(Message::PriceChanged),
                    text_input("Item Tax Group", &state.tax_group).on_input(Message::TaxGroupChanged),
                    column![checkbox("Overide Tax", state.tax_overide).on_toggle(Message::TaxOverideChanged).spacing(2)].spacing(10).padding(10),
                ],
                row![
                    button("Add Item").on_press(Message::NewItem)
                    .width(Length::Shrink),
                ],
            ]
        ].into()
    }
}


fn validate_i64(input: &str) -> bool {
    input.parse::<i64>().is_ok()
}

fn convert_to_i64(input: &str) -> i64 {
    input.parse::<i64>().unwrap_or(0)
}

fn validate_f64(input: &str) -> bool {
    input.parse::<f64>().is_ok()
}

fn convert_to_f64(input: &str) -> f64 {
    input.parse::<f64>().unwrap_or(0.0)
}




/* #[derive(Default, Clone)]
struct Receipt {
    items: Vec<Item>,
    service_charge: f64,
    gratuity: f64,
    service_charge_taxable: bool,
    gratuity_taxable: bool,
    tax_rate: f64, // Tax rate as a percentage (e.g., 0.10 for 10%)
} 

#[derive(Clone)]
struct Item {
    name: String,
    price: f64,
    quantity: u32,
    tax_override: bool, // true means no tax applies to this item
}

*/