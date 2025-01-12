use iced::widget::{button, column, row, text, text_input, Container};
use iced::{Color, Element, Length};
use std::hash::{Hash, Hasher};

use crate::core::tax_group::TaxGroup;
use crate::ui::custom_appearances::{pos_table_header, pos_table_row};
use crate::ui::gratuity_list_view::{Message, GratuityEditState};

#[derive(Debug, Clone, PartialEq)]
pub struct Gratuity {
    pub id: i64,
    pub name: String,
    pub percent: f64,
    pub tax_group: TaxGroup,
    pub tax_overide: bool,
}

impl Gratuity {
    pub fn new(id: i64, name: String, percent: f64, tax_group: TaxGroup, tax_overide: bool) -> Self {
        let tax_as_percentage = percent * 100.0;
        
        Gratuity {
            id: id,
            name: name,
            percent: percent,
            tax_group: tax_group,
            tax_overide: tax_overide,
        }
    }
}

impl Default for Gratuity {
    fn default() -> Self {
        Self {
            id: 1,
            name: "Banquet".to_string(),
            percent: 15.5,
            tax_group: TaxGroup::default(),
            tax_overide: false,
        }
    }
}

impl Eq for Gratuity {}

impl Hash for Gratuity {
    fn hash<H: Hasher>(&self, state: &mut H){
        self.id.hash(state);
        self.name.hash(state);
        self.tax_group.hash(state);
        self.tax_overide.hash(state);

        let percent_bits = self.percent.to_bits();
        percent_bits.hash(state);
    }
}


pub fn create_gratuity_table(gratuities: Vec<Gratuity>, edit_states: &mut std::collections::HashMap<i64, GratuityEditState>) -> Element<'static, Message> {
    
    let header = row![
        table_cell("ID".to_string(), true, 75_f32, 0, false, "".to_string(), edit_states),
        table_cell("Name".to_string(), true, 150_f32, 0, false, "".to_string(), edit_states),
        table_cell("Percent".to_string(), true, 100_f32, 0, false, "".to_string(), edit_states),
        table_cell("Tax Group".to_string(),true, 150_f32, 0, false, "".to_string(), edit_states),
        table_cell("Taxed".to_string(),true, 75_f32, 0, false, "".to_string(), edit_states),
        table_cell("Actions".to_string(), true, 200_f32, 0, false, "".to_string(), edit_states),
    ]
    .spacing(1)
    .padding(2)
    .into(); 

    // Table rows
    let rows: Element<Message> = column(
        std::iter::once(header)
            .chain(
                gratuities.into_iter().map(|gratuity| {
                    let is_editing = edit_states.get(&gratuity.id)
                        .map( |state| state.is_editing)
                        .unwrap_or(false);

                    row![
                    table_cell(gratuity.id.to_string(), false, 75_f32, gratuity.id, false, "id".to_string(), edit_states),
                    table_cell(gratuity.name.clone(), false, 150_f32, gratuity.id, is_editing, "name".to_string(), edit_states),
                    table_cell(format!("${:.2}", gratuity.percent), false, 100_f32, gratuity.id, is_editing, "percent".to_string(), edit_states),
                    table_cell(gratuity.tax_group.name.clone(),false, 150_f32, 0, false, "tax_group".to_string(), edit_states),
                    table_cell(gratuity.tax_overide.to_string(),false, 75_f32, 0, false, "tax_overide".to_string(), edit_states),
                    table_cell_with_action(
                        gratuity.id, 
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
    gratuity_id: i64,
    is_editing: bool,
    field_name: String,
    edit_states: &std::collections::HashMap<i64, GratuityEditState>,
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

            let current_value = if let Some(edit_state) = edit_states.get(&gratuity_id) {
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
                    Message::EditField(gratuity_id, field_name.to_string(), new_value)
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
pub fn table_cell_with_action(gratuity_id: i64, width: f32, is_editing: bool) -> Element<'static, Message> {
    let edit_save_button = if is_editing {
        button("Save")
            .on_press(Message::SaveGratuity(gratuity_id))
            .width(Length::Fill)
            .padding(2)
    } else {
        button("Edit")
            .on_press(Message::ToggleEditMode(gratuity_id, true))
            .width(Length::Fill)
            .padding(2)
    };

    let delete_cancel_button = if is_editing {
        button("Cancel")
            .on_press(Message::ToggleEditMode(gratuity_id, false))
            .width(Length::Fill)
            .padding(2)
    } else {
        button("Delete")
            .on_press(Message::DeleteGratuity(gratuity_id))
            .width(Length::Fill)
            .padding(2)
    };

    row![edit_save_button, delete_cancel_button]
        .spacing(1)
        .width(Length::Fixed(width))
        .align_y(iced::Alignment::Center)
        .into()

}