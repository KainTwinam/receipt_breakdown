use iced::widget::{column, row, text, button, Container};
use iced::{Color, Element, Length};
use std::hash::{Hash, Hasher};

use crate::ui::custom_appearances::{pos_table_header, pos_table_row};
use crate::ui::item_list_view::Message;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Tax {
    pub id: i64,
    pub percent: f64,
    pub name: String,
}

impl Tax {
    pub fn new(id: i64, percent: f64, name: String) -> Self {
        let tax_as_percentage = percent * 100.0;
        
        Tax {
            id: id,
            percent: percent,
            name: name
        }
    }
}

impl Eq for Tax {}

impl Hash for Tax {
    fn hash<H: Hasher>(&self, state: &mut H){
        self.id.hash(state);
        self.name.hash(state);

        let percent_bits = self.percent.to_bits();
        percent_bits.hash(state);
    }
}


pub fn create_items_table(taxes: Vec<Tax>, edit_states: &mut std::collections::HashMap<i64, bool>) -> Element<'static, Message> {
    // Table header
    let header = row![
        table_cell("ID", true, 100_f32),
        table_cell("Name", true, 200_f32),
        table_cell("Price", true, 100_f32),
        table_cell("Actions", true, 200_f32)
    ]
    .spacing(1)
    .padding(2)
    .into(); // TODO, create row styles

    // Table rows
    let rows: Element<Message> = column(
        std::iter::once(header)
            .chain(
                taxes.into_iter().map(|tax| {

                    let id_cell = table_cell(&tax.id.to_string(), false, 100_f32);
                    let name_cell = table_cell(&tax.name.clone(), false, 200_f32);
                    let percent_cell = table_cell(&format!("${:.2}", tax.percent), false, 100_f32);
                    let action_cell = table_cell_with_action(
                        tax.id, 
                        210_f32,
                        edit_states.get(&tax.id).cloned().unwrap_or(false),);

                    row![id_cell, name_cell, percent_cell, action_cell]
                    .spacing(1)
                    .padding(2)
                    .into()
                })
            )
            .collect::<Vec<_>>(),
    )
    .into();

    Container::new(rows)
        .width(Length::Fill)
        .height(Length::Shrink)
        .into()
}

// Helper function for standard table cells
pub fn table_cell(content: &str, is_header: bool, width: f32) -> Element<'static, Message> {
    
    let text_element = text(content.to_string())
        .color(Color::BLACK)
        .size(if is_header { 14 } else { 12 })
        .width(Length::Fixed(width))
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center);

    Container::new(text_element)
        .padding(5)
        .style(
            move |theme| if is_header { 
                pos_table_header(theme)
            } else { pos_table_row(theme)
        })
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center)
        .into()
}

// Helper function for cells with actions
pub fn table_cell_with_action(tax_id: i64, width: f32, is_editing: bool) -> Element<'static, Message> {
    let edit_save_button = if is_editing {
        button("Save")
            .on_press(Message::SaveItem(tax_id))
            .width(Length::Fill)
            .padding(2)
    } else {
        button("Edit")
            .on_press(Message::ToggleEditMode(tax_id, true))
            .width(Length::Fill)
            .padding(2)
    };

    let delete_cancel_button = if is_editing {
        button("Cancel")
            .on_press(Message::ToggleEditMode(tax_id, false))
            .width(Length::Fill)
            .padding(2)
    } else {
        button("Delete")
            .on_press(Message::DeleteItem(tax_id))
            .width(Length::Fill)
            .padding(2)
    };

    row![edit_save_button, delete_cancel_button]
        .spacing(1)
        .width(Length::Fixed(width))
        .align_y(iced::Alignment::Center)
        .into()

}