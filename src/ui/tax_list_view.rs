use iced::{Element, Length, Task};
use iced::widget::column;

use crate::core::tax::{Tax, create_taxes_table};

#[derive(Debug, Clone)]
pub struct TaxView {
    taxes: Vec<Tax>,
    edit_states: std::collections::HashMap<i64, TaxEditState>,
}

#[derive(Debug, Clone)]
pub enum Message {
    //modify tax list and states
    DeleteTax(i64),
    SaveTax(i64),
    NewTax(Tax),
    ToggleEditMode(i64, bool),
    EditField(i64, String, String)
}

impl TaxView {
    pub fn new() -> Self {
        TaxView {
            taxes: Vec::new(),
            edit_states: std::collections::HashMap::new(),
        }
    }

    pub fn update(state: &mut Self, message: Message) -> Task<Message> {
        match message {
            //modify tax list and states
            Message::DeleteTax(tax_id) => {
                state.taxes.retain(|x| x.id != tax_id);
                state.edit_states.remove(&tax_id);

                Task::none()
            }
            Message::SaveTax(tax_id) => {
                if let Some(tax) = state.taxes.iter_mut().find(|i| i.id == tax_id) {
                    if let Some(edit_state) = state.edit_states.get(&tax_id) {
                        tax.name = edit_state.name.clone();
                        if let Ok(percent) = edit_state.percent.parse::<f64>() {
                            tax.percent = percent;
                        }
                    }
                }

                Task::done(Message::ToggleEditMode(tax_id, false))
            }
            Message::NewTax(new_tax) => {
                state.taxes.push(new_tax);

                Task::none()
            }
            Message::ToggleEditMode(tax_id, editing) => {

                if editing {
                    if let Some(tax) = state.taxes.iter().find(|i| i.id == tax_id) {
                        state.edit_states.insert(tax_id, TaxEditState {
                            name: tax.name.clone(),
                            percent: tax.percent.to_string(),
                            is_editing: true,
                        });
                    } 
                } else {
                    state.edit_states.remove(&tax_id);
                }

                Task::none()
            }
            Message::EditField(tax_id, field_name, new_value) => {
                if let Some(edit_state) = state.edit_states.get_mut(&tax_id) {
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
            create_taxes_table(state.taxes.clone(), &mut state.edit_states.clone())
        ].height(Length::Fill).into()
    }
}

#[derive(Debug, Clone)]
pub struct TaxEditState {
    pub name: String,
    pub percent: String,
    pub is_editing: bool,
}