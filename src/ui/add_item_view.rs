use iced::{Element, Length, Task};
use iced::widget::{button, checkbox, column, row, text, text_input};

use crate::core::{
    items::Item,
    quantity::Quantity,
    service_charge::ServiceCharge,
    gratuity::Gratuity,
    tax::Tax,
    tax_group::TaxGroup,
    total::Total,
    calculations::{
        validate_f64,
        convert_to_f64,
        validate_i64,
        convert_to_i64
    },
};

use crate::ui::custom_appearances;
use custom_appearances::validator;

#[derive(Debug, Clone)]
pub struct AddItemForm {
    item_id: validator::Input,
    item_name: String,
    category: String,
    price: validator::Input,
    tax_group: String,
    tax_overide: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Submit,

    ItemIdChanged(validator::Message),
    ItemNameChanged(String),
    CategoryChanged(String),
    PriceChanged(validator::Message),
    TaxGroupChanged(String),
    TaxOverideChanged(bool),
}

pub enum Action {
    AddNewItem(Item)
}


impl AddItemForm {
    pub fn new() -> Self {
        AddItemForm {
            item_id: validator::Input::default(),
            item_name: String::new(),
            category: String::new(),
            price: validator::Input::default(),
            tax_group: String::new(),
            tax_overide: false,
        }
    }

    pub fn update(state: &mut Self, message: Message) -> Option<Action> {
        match message {
            Message::Submit => {
                let item_id: i64 = if validate_i64(&state.item_id.value)
                {
                    convert_to_i64(&state.item_id.value)
                } else { 0 };

                let price: f64 = if validate_f64(&state.price.value)
                {
                    convert_to_f64(&state.price.value)
                } else { 0.0 };

                let item_name = &state.item_name;

                let item_category = &state.category;

                //create sales tax to add to tax group
                let sales_tax = Tax::new(1, 0.085, "Sales Tax".to_string());

                //add sales tax to a new Vec 'taxes'
                let mut taxes = Vec::new();
                &taxes.push(sales_tax);

                //create new taxgroup with the sales tax
                let tax_group = TaxGroup::new(1, "Default".to_string(), taxes);


                let new_item = Item::new(item_id, item_name.to_string(), item_category.to_string(), price, tax_group, state.tax_overide);

                Some(Action::AddNewItem(new_item))
            }
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

                None
            }
            Message::ItemNameChanged(name) => {
                println!("Item Name field Changed");
                state.item_name = name;

                None
            }
            Message::CategoryChanged(category) => {
                println!("Item Category field Changed");
                state.category = category;

                None
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
            
                None
            }
            Message::TaxGroupChanged(taxgroup) => {
                state.tax_group = taxgroup;

                None
            }
            Message::TaxOverideChanged(taxoveride) => {
                state.tax_overide = taxoveride;
                None
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
                    text_input("Item Name", &state.item_name).on_input(Message::ItemNameChanged).id(format!("ItemName")),
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
                    button("Submit").on_press(Message::Submit)
                    .width(Length::Shrink),
                ],
            ]
        ].into()
    }
}