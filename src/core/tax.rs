use iced::widget::{button,column, row, text, text_input, Container};
use iced::{Color, Element, Length};
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};

use crate::ui::custom_appearances::{pos_table_header, pos_table_row};
use crate::ui::tax_list_view::{Message, TaxEditState};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PartialOrd)]
pub struct Tax {
    pub id: i64,
    pub name: String,
    pub percent: f64,
}

impl Tax {
    pub fn new(id: i64, name: String, percent: f64, ) -> Self {
        let tax_as_percentage = percent * 100.0;
        
        Tax {
            id: id,
            name: name,
            percent: percent,
        }
    }
}

impl Default for Tax {
    fn default() -> Self {
        Self {
            id: 1,
            name: "default".to_string(),
            percent: 7.75, 
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


pub fn create_taxes_table(taxes: Vec<Tax>, edit_states: &mut std::collections::HashMap<i64, TaxEditState>) -> Element<'static, Message> {
    // Table header
    let header = row![
        table_cell("ID".to_string(), true, 100_f32, 0, false, "".to_string(), edit_states),
        table_cell("Name".to_string(), true, 200_f32, 0, false, "".to_string(), edit_states),
        table_cell("Percent".to_string(), true, 100_f32, 0, false, "".to_string(), edit_states),
        table_cell("Actions".to_string(), true, 200_f32, 0, false, "".to_string(), edit_states)
    ]
    .spacing(1)
    .padding(2)
    .into(); // TODO, create row styles

    // Table rows
    let rows: Element<Message> = column(
        std::iter::once(header)
            .chain(
                taxes.into_iter().map(|tax| {
                    let is_editing = edit_states.get(&tax.id)
                        .map( |state| state.is_editing)
                        .unwrap_or(false);

                    row![
                    table_cell(tax.id.to_string(), false, 100_f32, tax.id, false, "id".to_string(), edit_states),
                    table_cell(tax.name.clone(), false, 200_f32, tax.id, is_editing, "name".to_string(), edit_states),
                    table_cell(format!("{:.2}%", tax.percent), false, 100_f32, tax.id, is_editing, "percent".to_string(), edit_states),
                    table_cell_with_action(
                        tax.id, 
                        210_f32,
                        is_editing)
                    ].spacing(1)
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
pub fn table_cell<'a>(
    content: String, 
    is_header: bool, 
    width: f32,
    tax_id: i64,
    is_editing: bool,
    field_name: String,
    edit_states: &std::collections::HashMap<i64, TaxEditState>,
) -> Element<'a, Message> {
    
    if is_header{

        let text_element = text(content.to_string())
        .color(Color::BLACK)
        .size(14)
        .width(Length::Fixed(width))
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center);

        Container::new(text_element)
        .padding(5)
        .style(pos_table_header)
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center)
        .into()

    } else {
        let element: Element<Message> = if is_editing {

            let current_value = if let Some(edit_state) = edit_states.get(&tax_id) {
                match field_name.as_str() {
                    "name" => edit_state.name.clone(),
                    "percent" => edit_state.percent.clone(),
                    _ => content.clone()
                }
            } else {
                content.clone()
            };

            text_input("", &current_value)
                .on_input(move |new_value| {
                    Message::EditField(tax_id, field_name.to_string(), new_value)
            }).size(12)
            .width(Length::Fixed(width))
            .into()

        } else {
            text(content)
                .color(Color::BLACK)
                .size(12)
                .width(Length::Fixed(width))
                .align_x(iced::Alignment::Center)
                .align_y(iced::Alignment::Center)
                .into()
        };

        Container::new(element)
            .padding(5)
            .style(pos_table_row)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .into()
    }
}

// Helper function for cells with actions
pub fn table_cell_with_action(tax_id: i64, width: f32, is_editing: bool) -> Element<'static, Message> {
    let edit_save_button = if is_editing {
        button("Save")
            .on_press(Message::SaveTax(tax_id))
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
            .on_press(Message::DeleteTax(tax_id))
            .width(Length::Fill)
            .padding(2)
    };

    row![edit_save_button, delete_cancel_button]
        .spacing(1)
        .width(Length::Fixed(width))
        .align_y(iced::Alignment::Center)
        .into()

}