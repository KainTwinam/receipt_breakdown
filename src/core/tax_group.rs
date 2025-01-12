use iced::widget::{button, column, row, text, text_input, Column, Container};
use iced::{Color, Element, Length};

use crate::core::tax::Tax;
use crate::ui::custom_appearances::{pos_table_header, pos_table_row, multiple_items};
use crate::ui::tax_group_list_view::{Message, TaxGroupEditState};




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaxGroup {
    pub id: i64,
    pub name: String,
    pub taxes: Vec<Tax>,
}

impl TaxGroup {
    pub fn new(id: i64, name: String, taxes: Vec<Tax>) -> Self {
        TaxGroup {
            id: id,
            name: name,
            taxes: taxes
        }
    }
    
    pub fn get_tax_percent(&self, tax_name: &str) -> Option<f64> {
        // Search for the tax by name and return its percent if found
        for tax in &self.taxes {
            if tax.name == tax_name {
                return Some(tax.percent);
            }
        }
        None
    }

}

impl Default for TaxGroup {
    fn default() -> Self {
        let mut tax_vec = Vec::new();
        tax_vec.push(Tax::default());

        Self {
            id: 1,
            name: "Default".to_string(),
            taxes: tax_vec
        }
    }
}

pub fn create_tax_group_table(tax_groups: Vec<TaxGroup>, edit_states: &mut std::collections::HashMap<i64, TaxGroupEditState>) -> Element<'static, Message> {
    // Table header
    let header = row![
        table_cell("ID".to_string(), true, 100_f32, 0, false, "".to_string(), edit_states),
        table_cell("Name".to_string(), true, 200_f32, 0, false, "".to_string(), edit_states),
        table_cell("Taxes".to_string(), true, 100_f32, 0, false, "".to_string(), edit_states),
        table_cell("Actions".to_string(), true, 200_f32, 0, false, "".to_string(), edit_states)
    ]
    .spacing(1)
    .padding(2)
    .into(); // TODO, create row styles

    // Table rows
    let rows: Element<Message> = column(
        std::iter::once(header)
            .chain(
                tax_groups.into_iter().map(|tax_group| {
                    let is_editing = edit_states.get(&tax_group.id)
                        .map( |state| state.is_editing)
                        .unwrap_or(false);

                    row![
                    table_cell(tax_group.id.to_string(), false, 100_f32, tax_group.id, false, "id".to_string(), edit_states),
                    table_cell(tax_group.name.clone(), false, 200_f32, tax_group.id, is_editing, "name".to_string(), edit_states),
                    table_cell(taxes_to_string(tax_group.taxes), false, 100_f32, tax_group.id, is_editing, "price".to_string(), edit_states),
                    //table_cell(taxes_to_string(tax_group.taxes), false, 100_f32, tax_group.id, is_editing, "price".to_string(), edit_states),
                    table_cell_with_action(
                        tax_group.id, 
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
    tax_group_id: i64,
    is_editing: bool,
    field_name: String,
    edit_states: &std::collections::HashMap<i64, TaxGroupEditState>,
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
            let current_value = if let Some(edit_state) = edit_states.get(&tax_group_id) {
                match field_name.as_str() {
                    "name" => edit_state.name.clone(),
                    "taxes" => {
                        let taxes_str = serde_json::to_string(&edit_state.taxes)
                            .unwrap_or_else(|_| "[]".to_string());
                        taxes_str
                    },
                    _ => content.clone()
                    }
            }   else { content.clone() };

            match field_name.as_str() {
                "name" => {
                    text_input("", &current_value)
                    .on_input(move |new_value| {
                        Message::EditField(tax_group_id, field_name.to_string(), new_value)
                }).size(12)
                .width(Length::Fixed(width))
                .into()
                }
                "taxes" => {
                    text_input("", &current_value)
                    .on_input(move |new_value| {
                        match serde_json::from_str::<Vec<Tax>>(&new_value) {
                            Ok(taxes) => Message::EditTaxes(tax_group_id, taxes),
                            Err(_) => Message::EditField(tax_group_id, field_name.to_string(), new_value)
                        }
                    })
                    .size(12)
                    .width(Length::Fixed(width))
                    .into()
                }
                _ => {
                    text(content)
                    .color(Color::BLACK)
                    .size(12)
                    .width(Length::Fixed(width))
                    .align_x(iced::Alignment::Center)
                    .align_y(iced::Alignment::Center)
                    .into()
                }
            }

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
pub fn table_cell_with_action(tax_group_id: i64, width: f32, is_editing: bool) -> Element<'static, Message> {
    let edit_save_button = if is_editing {
        button("Save")
            .on_press(Message::SaveTaxGroup(tax_group_id))
            .width(Length::Fill)
            .padding(2)
    } else {
        button("Edit")
            .on_press(Message::ToggleEditMode(tax_group_id, true))
            .width(Length::Fill)
            .padding(2)
    };

    let delete_cancel_button = if is_editing {
        button("Cancel")
            .on_press(Message::ToggleEditMode(tax_group_id, false))
            .width(Length::Fill)
            .padding(2)
    } else {
        button("Delete")
            .on_press(Message::DeleteTaxGroup(tax_group_id))
            .width(Length::Fill)
            .padding(2)
    };

    row![edit_save_button, delete_cancel_button]
        .spacing(1)
        .width(Length::Fixed(width))
        .align_y(iced::Alignment::Center)
        .into()

}


fn taxes_to_string(taxes: Vec<Tax>) -> String {
    
    let tax_str = taxes.iter().enumerate().map(|(i, tax)|
        if i == taxes.len() -1 {
            tax.name.clone()
        } else {
            format!("{},", tax.name.clone())
        }
        
    ).collect();

    tax_str
}

fn create_tax_buttons(taxes: Option<Vec<Tax>>) -> Element<'static, Message> {
    let mut column = Column::new();

    if let Some(taxes) = taxes {
        for tax in taxes {
            column = column.push(button(text(tax.name.clone())).on_press(Message::TaxPressed(tax.id)));
        }
    };

    column.into()

}