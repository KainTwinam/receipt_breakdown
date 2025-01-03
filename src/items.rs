use iced::widget::{column, row, text, button, Container};
use iced::{Color, Element, Length};
use std::hash::{Hash, Hasher};

use crate::TaxGroup;
use crate::ui::custom_appearances::{pos_table_header, pos_table_row};
use crate::ui;
use crate::ui::Message;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub price: f64,
    pub tax_group: TaxGroup,
    pub tax_overide: bool
}

impl Item {
    pub fn new(id: i64, name: String, category: String, price: f64, tax_group: TaxGroup, tax_overide: bool) -> Self {
//        println!("==Creating new Item==");
//        println!("id: {}", id);
//        println!("name: {}", name);
//        println!("percent: ${}", price);
//        println!("tax group: {}", tax_group.name);
//        println!("tax_overide: {}", tax_overide);
//        println!("=========================");
//        println!("");
        
        Item {
            id: id,
            name: name,
            category: "Default".to_string(),
            price: price,
            tax_group: tax_group,
            tax_overide: tax_overide,
        }
    }
}

impl Eq for Item {}

impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H){
        self.id.hash(state);
        self.name.hash(state);
        self.tax_group.hash(state);
        self.tax_overide.hash(state);
        self.category.hash(state);

        let price_bits = self.price.to_bits();
        price_bits.hash(state);
    }
}

pub fn items_view<'a>(item_vec: Result<Vec<Item>, String>, mut edit_state: std::collections::HashMap<i64, bool>) -> Element<'a, Message> {

    column![
        match item_vec {
            Ok(items) => {
                let item_table: Element<'a, Message> = column![
                    row![
                        text("Items").size(20),
                        button("Add Item").on_press(Message::NewItem)
                            .width(Length::Fill)
                            .padding(2),
                    ],
                    create_items_table(items, &mut edit_state)
                ].into();
    
                item_table
            },
            Err(e)=> {
                text(format!("Error loading Items: {}", e)).into()
            }
        }
    ].spacing(2).into()
    
}

pub fn create_items_table(items: Vec<Item>, edit_states: &mut std::collections::HashMap<i64, bool>) -> Element<'static, Message> {
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
                items.into_iter().map(|item| {

                    let id_cell = table_cell(&item.id.to_string(), false, 100_f32);
                    let name_cell = table_cell(&item.name.clone(), false, 200_f32);
                    let price_cell = table_cell(&format!("${:.2}", item.price), false, 100_f32);
                    let action_cell = table_cell_with_action(
                        item.id, 
                        210_f32,
                        edit_states.get(&item.id).cloned().unwrap_or(false),);

                    row![id_cell, name_cell, price_cell, action_cell]
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
pub fn table_cell_with_action(item_id: i64, width: f32, is_editing: bool) -> Element<'static, Message> {
    let edit_save_button = if is_editing {
        button("Save")
            .on_press(Message::SaveItem(item_id))
            .width(Length::Fill)
            .padding(2)
    } else {
        button("Edit")
            .on_press(Message::ToggleEditMode(item_id, true))
            .width(Length::Fill)
            .padding(2)
    };

    let delete_cancel_button = if is_editing {
        button("Cancel")
            .on_press(Message::ToggleEditMode(item_id, false))
            .width(Length::Fill)
            .padding(2)
    } else {
        button("Delete")
            .on_press(Message::DeleteItem(item_id))
            .width(Length::Fill)
            .padding(2)
    };

    row![edit_save_button, delete_cancel_button]
        .spacing(1)
        .width(Length::Fixed(width))
        .align_y(iced::Alignment::Center)
        .into()

}