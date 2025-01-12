use iced::widget::{button,column, row, text, text_input, Container};
use iced::{Color, Element, Length};
use std::collections::HashMap;
use std::marker::PhantomData;
use serde::{Serialize, Deserialize};

use crate::ui::custom_appearances::{pos_table_header, pos_table_row};


pub fn create_table<T, Renderer>(
    items: Vec<T>,
    state: &mut TableState<T>,
    theme: &iced::Theme,
) -> Element<'static, TableMessage>
where
    T: Table + Clone + 'static,
    Renderer: iced::Renderer + 'static,
{

    let columns: Vec<ColumnDef> = T::get_columns()
        .into_iter()
        .map(|col| ColumnDef {
            name: col.name.to_string(),
            width: col.width,
            editable: col.editable,
        }).collect();

    let header = row(
        columns.iter().map(|col| {
            Container::new(
                text(col.name.clone())
                .size(14)
                .width(Length::Fixed(col.width))
            )
            .padding(5)
            .style(pos_table_header)
            .into()
        })
        .chain(std::iter::once(
            Container::new(text("Actions".to_string()))
                .width(Length::Fixed(210.0))
                .padding(5)
                .style(pos_table_header)
                .into()
        ))
    )
    .spacing(1)
    .padding(2);

    let rows = items.into_iter().map(move |item| {
        let item_id = item.id();
        let is_editing = state.editing_id == Some(item_id);
    
        row(
            columns.iter().map({
                let item = item.clone();
                move |col| {
                    Container::<TableMessage, iced::Theme, iced::Renderer>::new(
                        if is_editing && col.editable {
                            let col_name = col.name.to_lowercase();
                            let value = state.edit_values
                                .get(&col_name)
                                .cloned()
                                .unwrap_or_else(|| item.get_field_value(&col_name));
    
                            iced::widget::text_input::<TableMessage, iced::Theme, iced::Renderer>("", &value)
                                .on_input({
                                    let col_name = col_name.clone();
                                    move |new_value| {
                                        TableMessage::EditField(item_id, col_name.clone(), new_value)
                                    }
                                })
                                .width(Length::Fixed(col.width))
                                .into()
                        } else {
                            let col_name = col.name.to_lowercase();
                            text::<iced::Theme, iced::Renderer>(item.get_field_value(&col_name))
                                .width(Length::Fixed(col.width))
                                .into()
                        }
                    )
                    .padding(5)
                    .style(pos_table_row)
                    .into()
                }
            })
            .chain(std::iter::once(create_action_buttons(item_id, is_editing)))
        )
        .spacing(1)
        .padding(2)
    });

/*     let header = create_header(&columns);

    let rows = items.into_iter().map(|item| {
        create_row(&item, &columns, state )
    }); */

    let table = column(
        std::iter::once(header.into())
            .chain(rows.map(Element::from))
            .collect::<Vec<_>>()
    );

    Container::new(table)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/* 
fn create_header(columns: &[ColumnDef]) -> Element<TableMessage> {
    row(
        columns.iter().map(|col| {
            Container::new(
                text(&col.name)
                    .size(14)
                    .width(Length::Fixed(col.width))
            )
            .padding(5)
            .style(pos_table_header)
            .into()
        })
        .chain(std::iter::once(
            Container::new(text("Actions"))
                .width(Length::Fixed(210.0))
                .padding(5)
                .style(pos_table_header)
                .into()
        ))
    )
    .spacing(1)
    .padding(2)
    .into()
}

fn create_row<T: Table>(
    item: &T,
    columns: &[ColumnDef],
    state: &TableState<T>
) -> Element<'static, TableMessage> {
    let id = item.id();
    let is_editing = state.editing_id == Some(id);

    row(
        columns.iter().map(|col| {
            let col_name = col.name.to_lowercase();

            let content: Element<TableMessage> = if is_editing && col.editable {
                let value = state.edit_values
                    .get(&col.name.to_lowercase())
                    .cloned()
                    .unwrap_or_else(|| item.get_field_value(&col_name));

                text_input::<TableMessage, iced::Theme, iced::Renderer>("", &value)
                    .on_input(move |new_value| {
                        TableMessage::EditField(id, col_name.clone(), new_value)
                    })
                    .width(Length::Fixed(col.width))
                    .into()
            } else {
                text(item.get_field_value(&col.name.to_lowercase()))
                    .width(Length::Fixed(col.width))
                    .into()
            };

            Container::new(content)
                .padding(5)
                .style(pos_table_row)
                .into()

        })
        .chain(std::iter::once(create_action_buttons(id, is_editing)))
    )
    .spacing(1)
    .padding(2)
    .into()
}
 */

fn create_action_buttons(id: i64, is_editing: bool) -> Element<'static, TableMessage> {
    let (edit_text, edit_msg) = if is_editing {
        ("Save", TableMessage::SaveRow(id))
    } else {
        ("Edit", TableMessage::StartEditing(id))
    };

    let (delete_text, delete_msg) = if is_editing {
        ("Cancel", TableMessage::CancelEditing(id))
    } else {
        ("Delete", TableMessage::DeleteRow(id))
    };

    row![
        button(edit_text).on_press(edit_msg),
        button(delete_text).on_press(delete_msg),
    ]
    .spacing(5)
    .width(Length::Fixed(210.0))
    .into()
}


#[derive(Debug, Clone)]
pub struct ColumnDef {
    pub name: String,
    pub width: f32,
    pub editable: bool,
}

pub trait Table {
    type Item: Serialize + Default;

    type EditState: BaseEditState;

    fn id(&self) -> i64;
    fn to_edit_state(&self) -> Self::EditState;

    fn get_columns() -> Vec<ColumnDef> {
        get_field_names::<Self::Item>()
            .into_iter()
            .map(|name| ColumnDef {
                name,
                width: 150.0,
                editable: true,
            })
            .collect()
    }

    fn get_field_value(&self, field_name: &str) -> String;
}

pub struct TableState<T> {
    editing_id: Option<i64>,
    edit_values: HashMap<String, String>,
    extra_state: HashMap<String, String>,
    _phantom: PhantomData<T>,
}

impl<T> TableState<T> {
    fn new() -> Self {
        Self {
            editing_id: None,
            edit_values: HashMap::new(),
            extra_state: HashMap::new(),
            _phantom: PhantomData,
        }
    }

    fn start_editing(&mut self, id: i64, initial_values: HashMap<String, String>) {
        self.editing_id = Some(id);
        self.edit_values = initial_values;
    }

    fn stop_editing(&mut self) {
        self.editing_id = None;
        self.edit_values.clear();
    }
}

pub trait BaseEditState {
    fn get_edit_values(&self) -> HashMap<String, String>;
    fn set_edit_values(&mut self, values: HashMap<String, String>);
}

#[derive(Debug, Clone)]
pub struct DefaultEditStateFields {
    pub is_editing: bool,
}

#[derive(Debug, Clone)]
pub enum TableMessage {
    EditField(i64, String, String),
    SaveRow(i64),
    StartEditing(i64),
    CancelEditing(i64),
    DeleteRow(i64),
}

fn get_field_names<T>() -> Vec<String>
where
    T: Serialize + Default
{
    let dummy = T::default();
    let map = serde_json::to_value(dummy).unwrap();
    match map {
        serde_json::Value::Object(map) => map.keys().cloned().collect(),
        _ => vec![],
    }
}