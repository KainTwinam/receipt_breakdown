use iced::{Element, Length, Task};
use iced::widget::{button, checkbox, column, row, text, text_input, Container};

use crate::core::{
    tax::Tax,
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
pub struct AddTaxForm {
    tax_id: validator::Input,
    tax_name: String,
    percent: validator::Input,
}

#[derive(Debug, Clone)]
pub enum Message {
    Submit,

    TaxIdChanged(validator::Message),
    TaxNameChanged(String),
    PercentChanged(validator::Message),
}

pub enum Action {
    AddNewTax(Tax)
}


impl AddTaxForm {
    pub fn new() -> Self {
        AddTaxForm {
            tax_id: validator::Input::default(),
            tax_name: String::new(),
            percent: validator::Input::default(),
        }
    }

    pub fn update(state: &mut Self, message: Message) -> Option<Action> {
        match message {
            Message::Submit => {
                let tax_id: i64 = if validate_i64(&state.tax_id.value)
                {
                    convert_to_i64(&state.tax_id.value)
                } else { 0 };

                let tax_percent: f64 = if validate_f64(&state.percent.value)
                {
                    convert_to_f64(&state.percent.value)
                } else { 0.0 };

                let tax_name = &state.tax_name;

                let new_tax = Tax::new(tax_id, state.tax_name.to_string(), tax_percent);

                Some(Action::AddNewTax(new_tax))
            }
            Message::TaxIdChanged(id) => {
                println!("ID Changed");
                match id {
                    validator::Message::RawInput(input) => {
                        state.tax_id.value = input;
                        state.tax_id.is_valid =
                            validator::validate(&state.tax_id.value, validate_i64);

                        if !state.tax_id.is_valid { 
                            state.tax_id.value = String::new();
                            state.tax_id.is_valid = true;
                            state.tax_id.placeholder = "Numbers Only".to_string();
                        } else { state.tax_id.placeholder = "".to_string() }
                    }
                    validator::Message::RawSubmit(input) => {
                        state.tax_id.value = input;
                        state.tax_id.is_valid =
                            validator::validate(&state.tax_id.value, validate_i64);

                    }
                }

                None
            }
            Message::TaxNameChanged(name) => {
                println!("Tax Name field Changed");
                state.tax_name = name;

                None
            }
            Message::PercentChanged(percent) => {
                println!("Percent Changed");
                match percent {
                    validator::Message::RawInput(input) => {
                        state.percent.value = input;
                        state.percent.is_valid =
                            validator::validate(&state.percent.value, validate_f64);

                        if !state.percent.is_valid { 
                            state.percent.value = String::new();
                            state.percent.is_valid = true;
                            state.percent.placeholder = "Numbers Only".to_string();
                        } else { state.percent.placeholder = "".to_string() }
                    }
                    validator::Message::RawSubmit(input) => {
                        state.percent.value = input;
                        state.percent.is_valid =
                            validator::validate(&state.percent.value, validate_f64);

                    }
                }
            
                None
            }
        }
    }


    pub fn view<'a>(state: &Self) -> Element<'static, Message>{
        Container::new(
            column![
                row![
                    text("Add Tax").size(16),
                ].padding(8),
                iced::widget::horizontal_rule(1),
                column![
                    text("ID").size(18),
                    validator::view(&state.tax_id.value.clone(), &state.tax_id.placeholder, true).map(Message::TaxIdChanged),
                ].padding(8),
                column![
                    text("Name").size(18),
                    text_input("", &state.tax_name).on_input(Message::TaxNameChanged).id(format!("tax-1")),
                ].padding(8),
                column![
                    text("Tax %").size(18),
                    validator::view(&state.percent.value.clone(), &state.percent.placeholder, true).map(Message::PercentChanged),
                ].padding(8),
                row![
                    iced::widget::horizontal_space().width(Length::Fill),
                    button("Submit").on_press(Message::Submit).width(Length::Shrink),
                    iced::widget::horizontal_space().width(Length::Fill),
                ].padding(8).width(130),
            ]
        )
        .width(130)
        .into()

    }
}