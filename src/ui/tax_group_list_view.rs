use iced::{Element, Length, Task};
use iced::widget::column;

use crate::core::tax_group::{TaxGroup, create_tax_group_table};
use crate::core::tax::Tax;

#[derive(Debug, Clone)]
pub struct TaxGroupView {
    tax_groups: Vec<TaxGroup>,
    edit_states: std::collections::HashMap<i64, TaxGroupEditState>,
}

#[derive(Debug, Clone)]
pub enum Message {
    DeleteTaxGroup(i64),
    SaveTaxGroup(i64),
    NewTaxGroup(TaxGroup),
    ToggleEditMode(i64, bool),
    EditField(i64, String, String),
    EditTaxes(i64, Vec<Tax>),
    TaxPressed(i64),
}

impl TaxGroupView {
    pub fn new() -> Self {
        
        let mut tax_group_vec = Vec::new();
        let tax_group = TaxGroup::default();
        tax_group_vec.push(tax_group);

        TaxGroupView{
            tax_groups: tax_group_vec,
            edit_states: std::collections::HashMap::new(),
        }
    }

    pub fn update(state: &mut Self, message: Message) -> Task<Message> {
        match message {
            Message::DeleteTaxGroup(tax_group_id) => {
                state.tax_groups.retain(|x| x.id != tax_group_id);
                state.edit_states.remove(&tax_group_id);

                Task::none()
            }
            Message::SaveTaxGroup(tax_group_id) => {
                if let Some(tax_group) = state.tax_groups.iter_mut().find(|i| i.id == tax_group_id) {
                    if let Some(edit_state) = state.edit_states.get(&tax_group_id) {
                        tax_group.name = edit_state.name.clone();
                        tax_group.taxes = edit_state.taxes.clone();
                    }
                }

                Task::done(Message::ToggleEditMode(tax_group_id, false))
            }
            Message::NewTaxGroup(new_tax_group) => {
                state.tax_groups.push(new_tax_group);

                Task::none()
            }
            Message::ToggleEditMode(tax_group_id, editing) => {
                if editing {
                    if let Some(tax_group) = state.tax_groups.iter().find(|i| i.id == tax_group_id) {
                        state.edit_states.insert(tax_group_id, TaxGroupEditState {
                            name: tax_group.name.clone(),
                            taxes: tax_group.taxes.clone(),
                            is_editing: true,
                        });
                    } 
                } else {
                    state.edit_states.remove(&tax_group_id);
                }
                
                Task::none()
            }
            Message::EditField(tax_group_id, field_name, new_value) => {
                if let Some(edit_state) = state.edit_states.get_mut(&tax_group_id) {
                    match field_name.as_str() {
                        "name" => edit_state.name = new_value,
                        _ => {}
                    }
                }

                Task::none()
            }
            Message::EditTaxes(tax_group_id, new_taxes) => {
                if let Some(edit_state) = state.edit_states.get_mut(&tax_group_id) {
                    edit_state.taxes = new_taxes;
                }

                Task::none()
            }
            Message::TaxPressed(tax_group_id) => {
/*                 if let Some(edit_state) = state.edit_states.get_mut(&tax_group_id) {
                    match field_name.as_str() {
                        "taxes" => {
                            
                        }
                        _ => {}
                    }
                } */

                Task::none()
            }
        }
    }

    pub fn view<'a>(state: &Self) -> Element<'static, Message> {

        column![
        create_tax_group_table(state.tax_groups.clone(), &mut state.edit_states.clone())
        ].height(Length::Fill).into()
    }
}

#[derive(Debug, Clone)]
pub struct TaxGroupEditState {
    pub name: String,
    pub taxes: Vec<Tax>,
    pub is_editing: bool,
}