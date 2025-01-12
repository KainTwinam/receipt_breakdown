use iced::alignment::Horizontal;
use iced::Alignment::Center;
use iced::{Alignment, Element, Length};
use iced::widget::{button, checkbox, column, container, row, text, text_input, Container};

use crate::core::{
    items::Item,
    tax::Tax,
    tax_group::TaxGroup,
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
                let sales_tax = Tax::default();

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
                        
                        if !state.item_id.is_valid { 
                            state.item_id.value = String::new();
                            state.item_id.is_valid = true;
                            state.item_id.placeholder = "Numbers Only".to_string();
                        } else { state.item_id.placeholder = "".to_string() }
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
                        
                        if !state.price.is_valid { 
                            state.price.value = String::new();
                            state.price.is_valid = true;
                            state.price.placeholder = "Numbers Only".to_string();
                        } else { state.price.placeholder = "".to_string() }
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
        Container::new(
            column![
                row![
                    text("Add Items").size(16),
                ].padding(8),
                iced::widget::horizontal_rule(1),
                column![
                    text("ID").size(18),
                    validator::view(&state.item_id.value.clone(), &state.item_id.placeholder, state.item_id.is_valid).map(Message::ItemIdChanged),
                ].padding(8),
                column![
                    text("Name").size(18),
                    text_input("", &state.item_name).on_input(Message::ItemNameChanged).id(format!("1")).width(120)
                    ].padding(8),
                column![
                    text("Category").size(18),
                    text_input("", &state.category).on_input(Message::CategoryChanged).width(120)
                    ].padding(8),
                column![
                    text("Price").size(18),
                    validator::view(&state.price.value, &state.price.placeholder, state.price.is_valid).map(Message::PriceChanged),
                ].padding(8),
                column![
                    text("Tax Group").size(18),
                    text_input("", &state.tax_group).on_input(Message::TaxGroupChanged).width(120)
                    ].padding(8),
                column![
                    checkbox("Tax Overide", state.tax_overide).on_toggle(Message::TaxOverideChanged).spacing(4)
                    ].spacing(8).padding(8),
                row![
                    iced::widget::horizontal_space().width(Length::Fill),
                    button("Add Item").on_press(Message::Submit).width(Length::Shrink),
                    iced::widget::horizontal_space().width(Length::Fill),
                ].padding(8).width(130),
            ]
        )
        .width(130)
        .into()
    }
}