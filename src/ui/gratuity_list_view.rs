use iced::{Element, Length, Task};
use iced::widget::{column, text};

use crate::core::gratuity::{Gratuity, create_gratuity_table};

#[derive(Debug, Clone)]
pub struct GratuityView {
    gratuitys: Vec<Gratuity>,
    edit_states: std::collections::HashMap<i64, GratuityEditState>,
}

#[derive(Debug, Clone)]
pub enum Message {
    //modify gratuity list and states
    DeleteGratuity(i64),
    SaveGratuity(i64),
    NewGratuity(Gratuity),
    ToggleEditMode(i64, bool),
    EditField(i64, String, String)
}

impl GratuityView {

    pub fn new() -> Self {

        let mut gratuity_vec = Vec::new();
        gratuity_vec.push(Gratuity::default());

        GratuityView {
            gratuitys: gratuity_vec,
            edit_states: std::collections::HashMap::new(),
        }
    }

    pub fn update(state: &mut Self, message: Message) -> Task<Message> {
        match message {
            //modify gratuity list and states
            Message::DeleteGratuity(gratuity_id) => {
                state.gratuitys.retain(|x| x.id != gratuity_id);
                state.edit_states.remove(&gratuity_id);

                Task::none()
            }
            Message::SaveGratuity(gratuity_id) => {
                if let Some(gratuity) = state.gratuitys.iter_mut().find(|i| i.id == gratuity_id) {
                    if let Some(edit_state) = state.edit_states.get(&gratuity_id) {
                        gratuity.name = edit_state.name.clone();
                        if let Ok(percent) = edit_state.percent.parse::<f64>() {
                            gratuity.percent = percent;
                        }
                    }
                }

                Task::done(Message::ToggleEditMode(gratuity_id, false))
            }
            Message::NewGratuity(new_gratuity) => {
                state.gratuitys.push(new_gratuity);

                Task::none()
            }
            Message::ToggleEditMode(gratuity_id, editing) => {

                if editing {
                    if let Some(gratuity) = state.gratuitys.iter().find(|i| i.id == gratuity_id) {
                        state.edit_states.insert(gratuity_id, GratuityEditState {
                            name: gratuity.name.clone(),
                            percent: gratuity.percent.to_string(),
                            tax_overide: gratuity.tax_overide,
                            is_editing: true,
                        });
                    } 
                } else {
                    state.edit_states.remove(&gratuity_id);
                }

                Task::none()
            }
            Message::EditField(gratuity_id, field_name, new_value) => {
                if let Some(edit_state) = state.edit_states.get_mut(&gratuity_id) {
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
            text("Gratuitiess").size(25),
            create_gratuity_table(state.gratuitys.clone(), &mut state.edit_states.clone())
        ].height(Length::Fill).padding(5).into()
    }
}

#[derive(Debug, Clone)]
pub struct GratuityEditState {
    pub name: String,
    pub percent: String,
    pub tax_overide: bool,
    pub is_editing: bool,
}