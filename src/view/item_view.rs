use crate::messages::{Message, ItemViewMessage};
use crate::types::type_definitions::Item;
use crate::widget::{Column, Container, Button, Element, Row, Text };
use crate::theme;

use iced::widget::{text_input, scrollable};
use iced::alignment::Horizontal;
use iced::Length;


pub struct ItemView {
    pub system_items: Vec<Item>,

    item_id: String,
    item_name: String,
    item_price: String,
    item_tax_group: String,
}

impl ItemView {
    pub fn new() -> Self {
        Self {
            system_items: Vec::new(),

            item_id: String::new(),
            item_name: String::new(),
            item_price: String::new(),
            item_tax_group: String::new(),
        }
    }
    //end of the 'new' function

    pub fn update(&mut self, message: ItemViewMessage) {
        match message {
            ItemViewMessage::ItemIdChanged(id) => self.item_id = id,
            ItemViewMessage::ItemNameChanged(name) => self.item_name = name,
            ItemViewMessage::ItemPriceChanged(price) => self.item_price = price,
            ItemViewMessage::ItemTaxGroupChanged(tax_group) => self.item_tax_group = tax_group,
            ItemViewMessage::AddSystemItem(item) => {
                let new_item = item;

                self.system_items.push(new_item);

                println!["{:?}", self.item_id];
                println!["{:?}", self.item_name];
                println!["{:?}", self.item_price];
                println!["{:?}", self.item_tax_group];

                //clear text inputs
                self.item_id = String::new();
                self.item_name = String::new();
                self.item_price = String::new();
                self.item_tax_group = String::new();
            }
            ItemViewMessage::EditSystemItem(item) => {
                //Edit the item -> How do I edit the active item??
                //self.item_id = item.id.to_string();
                //self.item_name = item.name.clone();
                //self.item_price = item.price.to_string();
                //self.item_tax_group = item.tax_group.clone();
            }
            ItemViewMessage::DeleteSystemItem(item) => {
                if self.system_items.len() == 0 {
                    //return Command::none();
                }
                if self.system_items.len() == 1 {
                    self.system_items.clear();
                }
                else {
                    self.system_items.remove(item.try_into().expect("Error removing item"));
                }
            }
        } // end of match statement
    }
    // end of 'update' function

    pub fn view(&self) -> Container<Message> {
        let add_item_button = Button::new(Text::new("Add Item"));

        let add_item_view_column = Column::new()
            .push(
                Row::new()
                    .push(Text::new("Item Id: "))
                    .push(
                        text_input("Item Id", &self.item_id)
                            .on_input(|input| Message::ItemView(ItemViewMessage::ItemIdChanged(input)))
                            .padding(5)
                    )
                    .push(Text::new("Item Name: "))
                    .push(
                        text_input("Item Name", &self.item_name)
                            .on_input(|input| Message::ItemView(ItemViewMessage::ItemNameChanged(input)))
                            .padding(5)
                    )
                    .push(Text::new("Item Price: "))
                    .push(
                        text_input("Item Price", &self.item_price)
                            .on_input(|input| Message::ItemView(ItemViewMessage::ItemPriceChanged(input)))
                            .padding(5)
                    )
                    .push(Text::new("Tax Group: "))
                    .push(
                        text_input("Tax Group", &self.item_tax_group)
                            .on_input(|input| Message::ItemView(ItemViewMessage::ItemTaxGroupChanged(input)))
                            .padding(5)
                    )
                    .push(
                        add_item_button
                            .on_press(
                                Message::ItemView( 
                                ItemViewMessage::AddSystemItem(
                                    Item::new(
                                        self.item_id.parse::<u64>().unwrap_or(1), 
                                        self.item_name.clone(), 
                                        self.item_price.parse::<f64>().unwrap_or(0.0), 
                                        self.item_tax_group.clone()
                                    ))
                                )
                            )
                            .style(theme::Button::Primary)
                            .padding(5)
                    )                   
            )
            .padding(5).spacing(12);
        
        let system_item_list = scrollable(
            Column::with_children(
                self.system_items
                    .iter()
                    .map(|item| {
                        Row::new()
                            .push(Text::new(item.id.to_string()))
                            .push(Text::new(item.name.clone()))
                            .push(Text::new(item.price.to_string()))
                            .push(Text::new(item.tax_group.clone()))
                            .push(Button::new(Text::new("Edit"))
                                .on_press(Message::ItemView(ItemViewMessage::EditSystemItem(item.id.clone())))
                                .style(theme::Button::MediaStart)
                                .padding(5))
                            .push(Button::new(Text::new("Delete"))
                                .on_press(Message::ItemView(ItemViewMessage::DeleteSystemItem(item.id.clone())))
                                .style(theme::Button::MediaStart)
                                .padding(5)
                            )
                            .spacing(10)
                            .width(Length::Fill)                                
                            .into()
                    })
                    .collect::<Vec<_>>(),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
        );

        let system_item_list_header = Column::new()
            .push(
                Row::new()
                    .push(Text::new("Item Id"))
                    .push(Text::new("Item Name"))
                    .push(Text::new("Item Price"))
                    .push(Text::new("Tax Group"))
                    .push(Text::new("Edit"))
                    .push(Text::new("Delete"))
                    .width(Length::Fill)
                    .spacing(10)
                    .padding(5)
            );

        let item_view_section = Column::with_children(
            vec![
                add_item_view_column.into(),
                system_item_list_header.into(),
                system_item_list.into(),
            ]
        )
        .spacing(10);

        Container::new(item_view_section)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            //.style(theme::Container::Black)
    }
    // end of 'view' function
}