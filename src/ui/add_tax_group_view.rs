use iced::{Element, Length};
use iced::widget::{button, column, row, text, text_input, Container};

use crate::core::{
    tax::Tax,
    tax_group::TaxGroup,
    calculations::{
        validate_i64,
        convert_to_i64
    },
};

use crate::ui::custom_appearances;
use custom_appearances::validator;

#[derive(Debug, Clone)]
pub struct AddTaxGroupForm {
    tax_group_id: validator::Input,
    tax_group_name: String,
    tax_group_taxes: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Submit,

    TaxGroupIdChanged(validator::Message),
    TaxGroupNameChanged(String),
    TaxGroupTaxesChanged(String,)
}

pub enum Action {
    AddNewTaxGroup(TaxGroup)
}


impl AddTaxGroupForm {
    pub fn new() -> Self {
        AddTaxGroupForm {
            tax_group_id: validator::Input::default(),
            tax_group_name: String::new(),
            tax_group_taxes: String::new(),
        }
    }

    pub fn update(state: &mut Self, message: Message) -> Option<Action> {
        match message {
            Message::Submit => {
                let tax_group_id: i64 = if validate_i64(&state.tax_group_id.value)
                {
                    convert_to_i64(&state.tax_group_id.value)
                } else { 0 };

                let tax_group_name = state.tax_group_name.clone();

                //let tax_group_tax = state.tax_group_taxes.clone();

                if parse_taxes(state.tax_group_taxes.clone()).is_ok() {
                    let taxes = parse_taxes(state.tax_group_taxes.clone()).ok().unwrap();
                    let new_tax_group = TaxGroup::new(tax_group_id, tax_group_name, taxes);

                    Some(Action::AddNewTaxGroup(new_tax_group))
                } else {
                    let mut tax_vec = Vec::new();
                    let default_tax = Tax::default();
                    tax_vec.push(default_tax);
                    let new_tax_group = TaxGroup::new(tax_group_id, tax_group_name, tax_vec);

                    Some(Action::AddNewTaxGroup(new_tax_group))
                }

            }
            Message::TaxGroupIdChanged(id) => {
                match id {
                    validator::Message::RawInput(input) => {
                        state.tax_group_id.value = input;
                        state.tax_group_id.is_valid =
                            validator::validate(&state.tax_group_id.value, validate_i64);

                        if !state.tax_group_id.is_valid { 
                            state.tax_group_id.value = String::new();
                            state.tax_group_id.is_valid = true;
                            state.tax_group_id.placeholder = "Numbers Only".to_string();
                        } else { state.tax_group_id.placeholder = "".to_string() }
                    }
                    validator::Message::RawSubmit(input) => {
                        state.tax_group_id.value = input;
                        state.tax_group_id.is_valid =
                            validator::validate(&state.tax_group_id.value, validate_i64);

                    }
                }

                None
            }
            Message::TaxGroupNameChanged(name) => {
                state.tax_group_name = name;

                None
            }
            Message::TaxGroupTaxesChanged(taxes) => {
                let cloned_taxes = taxes.clone();

                match parse_taxes(cloned_taxes) {
                    Ok(parsed_taxes) => {
                        state.tax_group_taxes = taxes;
                    }
                    Err(e) => {
                        //store error?
                    }
                }
                None
            }
        }
    }


    pub fn view<'a>(state: &Self) -> Element<'static, Message>{

        //create sales tax to add to tax group
        let sales_tax = Tax::default();
    
        //add sales tax to a new Vec 'taxes'
        let mut taxes = Vec::new();
        &taxes.push(sales_tax);
    

        Container::new(
        column![
            column![
                row![
                    text("Add Tax Group").size(16),
                ].padding(8),
                iced::widget::horizontal_rule(1),
                column![
                    text("ID").size(18),
                    validator::view(&state.tax_group_id.value.clone(), &state.tax_group_id.placeholder.clone(), true).map(Message::TaxGroupIdChanged),
                ].padding(8),

                column![
                    text("Name").size(18),
                    text_input("", &state.tax_group_name).on_input(Message::TaxGroupNameChanged).width(120),
                ].padding(8),
                column![
                    text("Taxes").size(18),
                    text_input("", &state.tax_group_taxes).on_input(Message::TaxGroupTaxesChanged).width(120),
                ].padding(8),
                row![
                    iced::widget::horizontal_space().width(Length::Fill),
                    button("Submit").on_press(Message::Submit).width(Length::Shrink),
                    iced::widget::horizontal_space().width(Length::Fill),
                ].padding(8).width(130),
            ]
        ]
        )
        .width(130)
        .into()
    }
}


fn parse_taxes(tax_string: String) -> Result<Vec<Tax>, serde_json::Error> {
    // Attempt to parse the JSON string into Vec<Tax>
    serde_json::from_str(&tax_string)
}