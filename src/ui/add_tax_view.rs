use iced::{Element, Length, Task};
use iced::widget::{button, checkbox, column, row, text, text_input};

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

                let new_tax = Tax::default();

                Some(Action::AddNewTax(new_tax))
            }
            Message::TaxIdChanged(id) => {
                println!("ID Changed");
                match id {
                    validator::Message::RawInput(input) => {
                        state.tax_id.value = input;
                        state.tax_id.is_valid =
                            validator::validate(&state.tax_id.value, validate_i64);
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
        let validator::Input {
            value,
            is_valid,
            placeholder,
        } = &state.tax_id;
    
        let validator::Input {
            value,
            is_valid,
            placeholder,
        } = &state.percent;
    
        column![
            column![
                row![
                    text("Add Tax").size(25),
                ],
                
                row![
                    column![
                        validator::view(&state.tax_id.value.clone(), &"Tax ID", true).map(Message::TaxIdChanged),
                        if state.tax_id.value.is_empty(){
                            "".into()
                        } else if state.tax_id.is_valid {
                            text("Perfect").style(text::primary)
                        } else {
                            text("Numbers Only").style(text::danger)
                        },
                    ],
                    text_input("Tax Name", &state.tax_name).on_input(Message::TaxNameChanged).id(format!("1")),
                    column![
                        validator::view(&state.percent.value, &"Tax percent", true).map(Message::PercentChanged),
                        if state.percent.value.is_empty(){
                            "".into()
                        } else if state.percent.is_valid {
                            text("Perfect").style(text::primary)
                        } else {
                            text("Numbers Only").style(text::danger)
                        },
                    ],
                row![
                    button("Submit").on_press(Message::Submit)
                    .width(Length::Shrink),
                ],
            ]]
        ].into()
    }
}