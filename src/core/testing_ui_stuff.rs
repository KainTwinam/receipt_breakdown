use iced::advanced::graphics::text::cosmic_text::ttf_parser::name;
use iced::widget::{button, column, row, text, text_input, Button, Column, Container, Row};
use iced::{Alignment, Color, Element, Length, Task};

use crate::core::tax::Tax;
use crate::ui::custom_appearances::{pos_table_header, pos_table_row, multiple_items};


#[derive(Debug, Clone)]
pub struct TestView {
    tax_groups: Vec<TaxGroup>,
    edit_states: std::collections::HashMap<i64, TestEditState>,
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

impl TestView {
    pub fn new() -> Self {

        //default tax vector
        let mut taxvec = Vec::new();

        //default tax vec
        let mut tax_group_vec = Vec::new();


        //default Taxes
        let tax = Tax::new(1_i64, "Sales".to_string(), 7.75_f64, );
        let tax2 = Tax::new(2_i64, "Liquor".to_string(), 8.75_f64, );
        
        //push to tax vec
        taxvec.push(tax);
        taxvec.push(tax2);

        //default tax group
        let tax_group = TaxGroup::new(1_i64, "Default".to_string(), taxvec);

        //push to tax group vec
        tax_group_vec.push(tax_group);

        TestView{
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
                        state.edit_states.insert(tax_group_id, TestEditState {
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
pub struct TestEditState {
    pub name: String,
    pub taxes: Vec<Tax>,
    pub is_editing: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
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

pub fn create_tax_group_table(tax_groups: Vec<TaxGroup>, edit_states: &mut std::collections::HashMap<i64, TestEditState>) -> Element<'static, Message> {
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
                    table_cell(taxes_to_string(tax_group.taxes), false, 100_f32, tax_group.id, is_editing, "taxes".to_string(), edit_states),
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
    edit_states: &std::collections::HashMap<i64, TestEditState>,
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
        let element: Element<'a, Message> = if is_editing {
            let current_value = if let Some(edit_state) = edit_states.get(&tax_group_id) {
                match field_name.as_str() {
                    "name" => edit_state.name.clone(),
                    "taxes" => {
                        let taxes_clone = edit_state.taxes.clone();

                        let tax_str: String = taxes_clone.into_iter().map(
                            |tax| format!("{}, ", tax.name)
                        ).collect();

                        tax_str

/*                         let taxes_str = serde_json::to_string(&edit_state.taxes)
                            .unwrap_or_else(|_| "[]".to_string());
                        taxes_str */
                    },
                    _ => content.clone()
                    }
            }   else { content.clone() };

            match field_name.as_str() {
                "name" => {
                    println!("Editing name: {}", content.clone());
                    text_input("", &current_value)
                    .on_input(move |new_value| {
                        Message::EditField(tax_group_id, field_name.to_string(), new_value)
                }).size(12)
                .width(Length::Fixed(width))
                .align_x(Alignment::Center)
                .into()
                }
                "taxes" => {
                    println!("Editing taxes.");
                    let tax_names: Vec<String> = current_value
                        .rsplit(", ")
                        .map(|s| s.to_string())
                        .collect();
                    for name in &tax_names {
                        println!("tax_names: {:?}", name);
                    }
                    

                    let tax_buttons = tax_names
                        .into_iter()
                        .filter(|s| !s.is_empty())
                        .map(|tax_name| {
                            button(text(tax_name)).on_press(Message::TaxPressed(tax_group_id)).into()
                        })
                        .collect::<Vec<_>>();


                    Container::new(Row::with_children(tax_buttons)).into()
                }
                _ => {
                    println!("Editing FieldName: {}", &field_name);
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


fn taxes_to_string(mut taxes: Vec<Tax>) -> String {

    taxes.sort_by_key(|tax| tax.name.clone());
    
    let tax_str = taxes.iter().enumerate().map(|(i, tax)|
        if i == taxes.len() -1 {
            tax.name.clone()
        } else {
            format!("{}, ", tax.name.clone())
        }
        
    ).collect();

    tax_str
}

/* 
fn create_tax_buttons(taxes: Vec<Tax>) -> Element<'static, Message> {
    let tax_buttons = taxes.into_iter()
        .filter(|s| !s.name.is_empty())
        .map( |tax|
            button(text(tax.name)).on_press(Message::TaxPressed(tax.id))
        ).collect();

        Container::new(Row::with_children(tax_buttons)).into()
}
 */