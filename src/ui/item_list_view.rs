use iced::{Element, Length, Task};
use iced::widget::{column, text};

use crate::core::items::{Item, create_items_table};

#[derive(Debug, Clone)]
pub struct ItemView {
    items: Vec<Item>,
    edit_states: std::collections::HashMap<i64, ItemEditState>,
}

#[derive(Debug, Clone)]
pub enum Message {
    //modify item list and states
    DeleteItem(i64),
    SaveItem(i64),
    NewItem(Item),
    ToggleEditMode(i64, bool),
    EditField(i64, String, String)
}

impl ItemView {

    pub fn new() -> Self {

        let mut item_vec = Vec::new();
        item_vec.push(Item::default());

        ItemView {
            items: item_vec,
            edit_states: std::collections::HashMap::new(),
        }
    }

    pub fn update(state: &mut Self, message: Message) -> Task<Message> {
        match message {
            //modify item list and states
            Message::DeleteItem(item_id) => {
                state.items.retain(|x| x.id != item_id);
                state.edit_states.remove(&item_id);

                Task::none()
            }
            Message::SaveItem(item_id) => {
                if let Some(item) = state.items.iter_mut().find(|i| i.id == item_id) {
                    if let Some(edit_state) = state.edit_states.get(&item_id) {
                        item.name = edit_state.name.clone();
                        if let Ok(price) = edit_state.price.parse::<f64>() {
                            item.price = price;
                        }
                    }
                }

                Task::done(Message::ToggleEditMode(item_id, false))
            }
            Message::NewItem(new_item) => {
                state.items.push(new_item);

                Task::none()
            }
            Message::ToggleEditMode(item_id, editing) => {

                if editing {
                    if let Some(item) = state.items.iter().find(|i| i.id == item_id) {
                        state.edit_states.insert(item_id, ItemEditState {
                            name: item.name.clone(),
                            price: item.price.to_string(),
                            category: item.category.clone(),
                            tax_overide: item.tax_overide,
                            is_editing: true,
                        });
                    } 
                } else {
                    state.edit_states.remove(&item_id);
                }

                Task::none()
            }
            Message::EditField(item_id, field_name, new_value) => {
                if let Some(edit_state) = state.edit_states.get_mut(&item_id) {
                    match field_name.as_str() {
                        "name" => edit_state.name = new_value,
                        "price" => edit_state.price = new_value,
                        "category" => edit_state.category = new_value,
                        _ => {}
                    }
                }

                Task::none()
            }
        }
    }

    pub fn view<'a>(state: &Self) -> Element<'static, Message>{

        column![
            text("Items").size(25),
            create_items_table(state.items.clone(), &mut state.edit_states.clone())
        ].height(Length::Fill).padding(5).into()
    }
}

#[derive(Debug, Clone)]
pub struct ItemEditState {
    pub name: String,
    pub price: String,
    pub category: String,
    pub tax_overide: bool,
    pub is_editing: bool,
}