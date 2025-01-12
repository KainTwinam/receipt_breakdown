use iced::alignment::Horizontal;
use iced::Alignment::Center;
use iced::{Alignment, Element, Length};
use iced::widget::{button, checkbox, column, container, row, text, text_input, Container};

use crate::core::{
    service_charge::ServiceCharge,
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
pub struct AddServiceChargeForm {
    service_charge_id: validator::Input,
    service_charge_name: String,
    percent: validator::Input,
    tax_group: String,
    tax_overide: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Submit,

    ServiceChargeIdChanged(validator::Message),
    ServiceChargeNameChanged(String),
    PercentChanged(validator::Message),
    TaxGroupChanged(String),
    TaxOverideChanged(bool),
}

pub enum Action {
    AddNewServiceCharge(ServiceCharge)
}


impl AddServiceChargeForm {
    pub fn new() -> Self {
        AddServiceChargeForm {
            service_charge_id: validator::Input::default(),
            service_charge_name: String::new(),
            percent: validator::Input::default(),
            tax_group: String::new(),
            tax_overide: false,
        }
    }

    pub fn update(state: &mut Self, message: Message) -> Option<Action> {
        match message {
            Message::Submit => {
                let service_charge_id: i64 = if validate_i64(&state.service_charge_id.value)
                {
                    convert_to_i64(&state.service_charge_id.value)
                } else { 0 };

                let percent: f64 = if validate_f64(&state.percent.value)
                {
                    convert_to_f64(&state.percent.value)
                } else { 0.0 };

                let service_charge_name = &state.service_charge_name;

                //create sales tax to add to tax group
                let sales_tax = Tax::default();

                //add sales tax to a new Vec 'taxes'
                let mut taxes = Vec::new();
                &taxes.push(sales_tax);

                //create new taxgroup with the sales tax
                let tax_group = TaxGroup::new(1, "Default".to_string(), taxes);

                let new_service_charge = ServiceCharge::new(service_charge_id, service_charge_name.to_string(),  percent, tax_group, state.tax_overide);

                Some(Action::AddNewServiceCharge(new_service_charge))
            }
            Message::ServiceChargeIdChanged(id) => {
                println!("ID Changed");
                match id {
                    validator::Message::RawInput(input) => {
                        state.service_charge_id.value = input;
                        state.service_charge_id.is_valid =
                            validator::validate(&state.service_charge_id.value, validate_i64);
                        
                        if !state.service_charge_id.is_valid { 
                            state.service_charge_id.value = String::new();
                            state.service_charge_id.is_valid = true;
                            state.service_charge_id.placeholder = "Numbers Only".to_string();
                        } else { state.service_charge_id.placeholder = "".to_string() }
                    }
                    validator::Message::RawSubmit(input) => {
                        state.service_charge_id.value = input;
                        state.service_charge_id.is_valid =
                            validator::validate(&state.service_charge_id.value, validate_i64);

                    }
                }

                None
            }
            Message::ServiceChargeNameChanged(name) => {
                println!("Item Name field Changed");
                state.service_charge_name = name;

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
                    text("Add Service Charges").size(16),
                ].padding(8),
                iced::widget::horizontal_rule(1),
                column![
                    text("ID").size(18),
                    validator::view(&state.service_charge_id.value.clone(), &state.service_charge_id.placeholder, state.service_charge_id.is_valid).map(Message::ServiceChargeIdChanged),
                ].padding(8),
                column![
                    text("Name").size(18),
                    text_input("", &state.service_charge_name).on_input(Message::ServiceChargeNameChanged).id(format!("1")).width(120)
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
                    button("Add Service Charge").on_press(Message::Submit).width(Length::Shrink),
                    iced::widget::horizontal_space().width(Length::Fill),
                ].padding(8).width(130),
            ]
        )
        .width(130)
        .into()
    }
}