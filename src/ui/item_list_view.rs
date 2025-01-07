use iced::{Element, Length, Task};
use iced::widget::column;

use crate::core::items::{Item, create_items_table};

#[derive(Debug, Clone)]
pub struct ItemView {
    //windows: HashMap<window::Id, Window>,
    items: Vec<Item>,
    edit_states: std::collections::HashMap<i64, bool>,
}

#[derive(Debug, Clone)]
pub enum Message {
    //modify item list and states
    DeleteItem(i64),
    SaveItem(i64),
    NewItem(Item),
    ToggleEditMode(i64, bool),
}

impl ItemView {
    pub fn new() -> Self {
        ItemView {
            items: Vec::new(),
            edit_states: std::collections::HashMap::new(),
        }
    }

    pub fn update(state: &mut Self, message: Message) -> Task<Message> {
        match message {
            //modify item list and states
            Message::DeleteItem(item_id) => {
                state.items.retain(|x| x.id != item_id);
                Task::none()
            }
            Message::SaveItem(item_id) => {
                println!("Saving Item!");
                for item in state.items.iter_mut(){
                    if item.id == item_id {

                    }
                }
                Task::done(Message::ToggleEditMode(item_id, false))
            }
            Message::NewItem(new_item) => {
                state.items.push(new_item);
                
                Task::none()
            }
            Message::ToggleEditMode(item_id, editing) => {
                if editing{
                    state.edit_states.insert(item_id, editing);
                } else {
                    state.edit_states.remove(&item_id);
                }
                Task::none()
            }
        }
    }

    pub fn view<'a>(state: &Self) -> Element<'static, Message>{

        column![
            create_items_table(state.items.clone(), &mut state.edit_states.clone())
        ].height(Length::Fill).into()
    }
}