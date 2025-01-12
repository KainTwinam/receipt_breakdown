use iced::{Element, Length, Task};
use iced::widget::{column, text};

use crate::core::service_charge::{ServiceCharge, create_service_charge_table};

#[derive(Debug, Clone)]
pub struct ServiceChargeView {
    service_charges: Vec<ServiceCharge>,
    edit_states: std::collections::HashMap<i64, ServiceChargeEditState>,
}

#[derive(Debug, Clone)]
pub enum Message {
    //modify service_charge list and states
    DeleteServiceCharge(i64),
    SaveServiceCharge(i64),
    NewServiceCharge(ServiceCharge),
    ToggleEditMode(i64, bool),
    EditField(i64, String, String)
}

impl ServiceChargeView {

    pub fn new() -> Self {

        let mut service_charge_vec = Vec::new();
        service_charge_vec.push(ServiceCharge::default());

        ServiceChargeView {
            service_charges: service_charge_vec,
            edit_states: std::collections::HashMap::new(),
        }
    }

    pub fn update(state: &mut Self, message: Message) -> Task<Message> {
        match message {
            //modify service_charge list and states
            Message::DeleteServiceCharge(service_charge_id) => {
                state.service_charges.retain(|x| x.id != service_charge_id);
                state.edit_states.remove(&service_charge_id);

                Task::none()
            }
            Message::SaveServiceCharge(service_charge_id) => {
                if let Some(service_charge) = state.service_charges.iter_mut().find(|i| i.id == service_charge_id) {
                    if let Some(edit_state) = state.edit_states.get(&service_charge_id) {
                        service_charge.name = edit_state.name.clone();
                        if let Ok(percent) = edit_state.percent.parse::<f64>() {
                            service_charge.percent = percent;
                        }
                    }
                }

                Task::done(Message::ToggleEditMode(service_charge_id, false))
            }
            Message::NewServiceCharge(new_service_charge) => {
                state.service_charges.push(new_service_charge);

                Task::none()
            }
            Message::ToggleEditMode(service_charge_id, editing) => {

                if editing {
                    if let Some(service_charge) = state.service_charges.iter().find(|i| i.id == service_charge_id) {
                        state.edit_states.insert(service_charge_id, ServiceChargeEditState {
                            name: service_charge.name.clone(),
                            percent: service_charge.percent.to_string(),
                            tax_overide: service_charge.tax_overide,
                            is_editing: true,
                        });
                    } 
                } else {
                    state.edit_states.remove(&service_charge_id);
                }

                Task::none()
            }
            Message::EditField(service_charge_id, field_name, new_value) => {
                if let Some(edit_state) = state.edit_states.get_mut(&service_charge_id) {
                    match field_name.as_str() {
                        "name" => edit_state.name = new_value,
                        "percent" => edit_state.percent = new_value,
                        _ => {}
                    }
                }

                Task::none()
            }
        }
    }

    pub fn view<'a>(state: &Self) -> Element<'static, Message>{

        column![
            text("Service Charges").size(25),
            create_service_charge_table(state.service_charges.clone(), &mut state.edit_states.clone())
        ].height(Length::Fill).padding(5).into()
    }
}

#[derive(Debug, Clone)]
pub struct ServiceChargeEditState {
    pub name: String,
    pub percent: String,
    pub tax_overide: bool,
    pub is_editing: bool,
}