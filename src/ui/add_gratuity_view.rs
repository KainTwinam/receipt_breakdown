use iced::alignment::Horizontal;
use iced::Alignment::Center;
use iced::{Alignment, Element, Length};
use iced::widget::{button, checkbox, column, container, row, text, text_input, Container};

use crate::core::{
    gratuity::Gratuity,
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
pub struct AddGratuityForm {
    gratuity_id: validator::Input,
    gratuity_name: String,
    percent: validator::Input,
    tax_group: String,
    tax_overide: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Submit,

    GratuityIdChanged(validator::Message),
    GratuityNameChanged(String),
    PercentChanged(validator::Message),
    TaxGroupChanged(String),
    TaxOverideChanged(bool),
}

pub enum Action {
    AddNewGratuity(Gratuity)
}


impl AddGratuityForm {
    pub fn new() -> Self {
        AddGratuityForm {
            gratuity_id: validator::Input::default(),
            gratuity_name: String::new(),
            percent: validator::Input::default(),
            tax_group: String::new(),
            tax_overide: false,
        }
    }

    pub fn update(state: &mut Self, message: Message) -> Option<Action> {
        match message {
            Message::Submit => {
                let gratuity_id: i64 = if validate_i64(&state.gratuity_id.value)
                {
                    convert_to_i64(&state.gratuity_id.value)
                } else { 0 };

                let percent: f64 = if validate_f64(&state.percent.value)
                {
                    convert_to_f64(&state.percent.value)
                } else { 0.0 };

                let gratuity_name = &state.gratuity_name;

                //create sales tax to add to tax group
                let sales_tax = Tax::default();

                //add sales tax to a new Vec 'taxes'
                let mut taxes = Vec::new();
                &taxes.push(sales_tax);

                //create new taxgroup with the sales tax
                let tax_group = TaxGroup::new(1, "Default".to_string(), taxes);

                let new_gratuity = Gratuity::new(gratuity_id, gratuity_name.to_string(), percent, tax_group, state.tax_overide);

                Some(Action::AddNewGratuity(new_gratuity))
            }
            Message::GratuityIdChanged(id) => {
                println!("ID Changed");
                match id {
                    validator::Message::RawInput(input) => {
                        state.gratuity_id.value = input;
                        state.gratuity_id.is_valid =
                            validator::validate(&state.gratuity_id.value, validate_i64);
                        
                        if !state.gratuity_id.is_valid { 
                            state.gratuity_id.value = String::new();
                            state.gratuity_id.is_valid = true;
                            state.gratuity_id.placeholder = "Numbers Only".to_string();
                        } else { state.gratuity_id.placeholder = "".to_string() }
                    }
                    validator::Message::RawSubmit(input) => {
                        state.gratuity_id.value = input;
                        state.gratuity_id.is_valid =
                            validator::validate(&state.gratuity_id.value, validate_i64);

                    }
                }

                None
            }
            Message::GratuityNameChanged(name) => {
                println!("Item Name field Changed");
                state.gratuity_name = name;

                None
            }
            Message::PercentChanged(percent) => {
                println!("percent Changed");
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
                    text("Add Gratuities").size(16),
                ].padding(8),
                iced::widget::horizontal_rule(1),
                column![
                    text("ID").size(18),
                    validator::view(&state.gratuity_id.value.clone(), &state.gratuity_id.placeholder, state.gratuity_id.is_valid).map(Message::GratuityIdChanged),
                ].padding(8),
                column![
                    text("Name").size(18),
                    text_input("", &state.gratuity_name).on_input(Message::GratuityNameChanged).id(format!("1")).width(120)
                    ].padding(8),
                column![
                    text("percent").size(18),
                    validator::view(&state.percent.value, &state.percent.placeholder, state.percent.is_valid).map(Message::PercentChanged),
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
                    button("Add Gratuity").on_press(Message::Submit).width(Length::Shrink),
                    iced::widget::horizontal_space().width(Length::Fill),
                ].padding(8).width(130),
            ]
        )
        .width(130)
        .into()
    }
}