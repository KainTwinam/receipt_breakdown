use crate::messages::Message;
use crate::types::type_definitions::Type;
use crate::types::type_definitions::{CheckType, Check, Gratuity, Item, RevenueCategory, ServiceCharge, TaxGroup, Tax, };
use crate::view::views::View;
use crate::theme;
use crate::components::theme::themes::Themes;
use crate::widget::{Column, Container, Button, Element, Row, Text };
use crate::widget::table::lib::table;
use crate::view::item_view::ItemView;

use iced::executor;
use iced::widget::{ checkbox, row, scrollable, text_input,};
use iced::{Application, Command, Length, };
use iced::alignment::{Alignment, Horizontal};
use iced_widget::runtime::system;

pub struct ReceiptCalculator {
    pub view: View,
    pub theme: Themes,
    pub types: Vec<Type>,

    // different view states
    item_view: ItemView,

    // System Data acting as the "in memory" database
    //pub system_items: Vec<Item>,
    pub system_check_types: Vec<CheckType>,
    pub system_checks: Vec<Check>,
    pub system_gratuities: Vec<Gratuity>,
    pub system_revenue_categories: Vec<RevenueCategory>,
    pub system_service_charges: Vec<ServiceCharge>,
    pub system_tax_groups: Vec<TaxGroup>,
    pub system_taxes: Vec<Tax>,
    
    check_type_id: String,
    check_type_name: String,
    check_type_description: String,
    default_check_type: bool,
    active_check_type: bool,

    check_id: String,
    check_name: String,

    gratuity_id: String,
    gratuity_name: String,
    gratuity_rate: String,
    gratuity_revenue_categories: String,
    gratuity_taxed: bool,

    //item_id: String,
    //item_name: String,
    //item_price: String,
    //item_tax_group: String,


}

impl Application for ReceiptCalculator {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = theme::Theme;

    fn new(_flags: ()) -> (ReceiptCalculator, Command<Message>) {
        (
            
            ReceiptCalculator {
                view: View::DefaultView,
                theme: Themes::Nord,
                types: vec![
                    Type::CheckType,
                    Type::Check,
                    Type::Gratuity,
                    Type::Item,
                    Type::RevenueCategory,
                    Type::ServiceCharge,
                    Type::TaxGroup,
                    Type::Tax,
                ],
                // initialize the item_view
                item_view: ItemView::new(),

                //system_items: Vec::new(),
                system_check_types: Vec::new(),
                system_checks: Vec::new(),
                system_gratuities: Vec::new(),
                system_revenue_categories: Vec::new(),
                system_service_charges: Vec::new(),
                system_tax_groups: Vec::new(),
                system_taxes: Vec::new(),

                check_type_id: String::new(),
                check_type_name: String::new(),
                check_type_description: String::new(),
                default_check_type: true,
                active_check_type: true,

                check_id: String::new(),
                check_name: String::new(),

                gratuity_id: String::new(),
                gratuity_name: String::new(),
                gratuity_rate: String::new(),
                gratuity_revenue_categories: String::new(),
                gratuity_taxed: true,

                //item_id: String::new(),
                //item_name: String::new(),
                //item_price: String::new(),
                //item_tax_group: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Piece of Shit - POS")
    }

    fn theme(&self) -> Self::Theme {
        theme::Theme(self.theme.palette()).clone()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message{
            Message::UpdateView(app_view) => {
                match app_view {
                    Type::CheckType => self.view = View::CheckTypeView,
                    Type::Check => self.view = View::CheckView,
                    Type::Gratuity => self.view = View::GratuityView,
                    Type::Item => self.view = View::ItemView,
                    Type::RevenueCategory => self.view = View::RevenueCategoryView,
                    Type::ServiceCharge => self.view = View::ServiceChargeView,
                    Type::TaxGroup => self.view = View::TaxGroupView,
                    Type::Tax => self.view = View::TaxView,
                };
            }
            Message::ItemView(item_view_message) => {
                self.item_view.update(item_view_message);
            }
            Message::CheckTypeIdChanged(value) => {
                self.check_type_id = value;
            }
            Message::CheckTypeNameChanged(value) => {
                self.check_type_name = value;
            }
            Message::CheckTypeDescriptionChanged(value) => {
                self.check_type_description = value;
            }
            Message::DefaultCheckTypeChanged(default_check_type) => {
                self.default_check_type = !self.default_check_type;
            }
            Message::ActiveCheckTypeChanged(active_check_type) => {
                self.active_check_type = !self.active_check_type;
            }
            Message::CheckIdChanged(value) => {
                self.check_id = value;
            }
            Message::CheckNameChanged(value) => {
                self.check_name = value;
            }
            Message::GratuityIdChanged(value) => {
                self.gratuity_id = value;
            }
            Message::GratuityNameChanged(value) => {
                self.gratuity_name = value;
            }
            Message::GratuityRateChanged(value) => {
                self.gratuity_rate = value;
            }
            Message::GratuityRevenueCategoriesChanged(value) => {
                self.gratuity_revenue_categories = value;
            }
            Message::GratuityTaxedChanged(gratuity_taxed) => {
                self.gratuity_taxed = !self.gratuity_taxed;
            }
/*             Message::ItemIdChanged(value) => {
                self.item_id = value;
            }
            Message::ItemNameChanged(value) => {
                self.item_name = value;
            }
            Message::ItemPriceChanged(value) => {
                self.item_price = value;
            }
            Message::ItemTaxGroupChanged(value) => {
                self.item_tax_group = value;
            }
            Message::AddSystemItem(item) => {
                let new_item = item;

                self.system_items.push(new_item);

                //clear text inputs
                self.item_id = String::new();
                self.item_name = String::new();
                self.item_price = String::new();
                self.item_tax_group = String::new();
            }
            Message::EditSystemItem(item) => {
                // Edit the item -> How do I edit the active item??
                //self.item_id = item.id.to_string();
                //self.item_name = item.name.clone();
                //self.item_price = item.price.to_string();
                //self.item_tax_group = item.tax_group.clone();
            }
            Message::DeleteSystemItem(item) => {
                if self.system_items.len() == 0 {
                    return Command::none();
                }
                if self.system_items.len() == 1 {
                    self.system_items.clear();
                }
                else {
                    println!("removing item: {:?}", item);
                    self.system_items.remove(item.try_into().expect("Error removing item"));
                }
            } */

//            _ => (),
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        
        // Start Left Row Widget
        let left_menu_buttons = Column::with_children(
            self.types
                .iter()
                .map(|a_type| {
                    let button_text: Text = Text::new( 
                        match a_type{
                            Type::CheckType => "Check Type".to_string(),
                            Type::Check => "Check".to_string(),
                            Type::Gratuity => "Gratuity".to_string(),
                            Type::Item => "Item".to_string(),
                            Type::RevenueCategory => "Revenue Category".to_string(),
                            Type::ServiceCharge => "Service Charge".to_string(),
                            Type::TaxGroup => "Tax Group".to_string(),
                            Type::Tax => "Tax".to_string(),
                    });
                    let button_text_button = Button::new(button_text).width(Length::Fixed(145_f32));
    
                    Row::new()
                    .push(button_text_button.on_press(Message::UpdateView(a_type.clone())))
                    .spacing(20)
                    .align_items(Alignment::Center)
                    .into()
                })
                .collect::<Vec<_>>(),
        )
        .spacing(5)
        .padding(8);
        let left_menu_scrollable = scrollable(left_menu_buttons).height(Length::Fill).width(Length::Shrink);    
        let left_menu_content = Column::new()
            .align_items(Alignment::Center)
            .spacing(10)
            .push(left_menu_scrollable);
        let left_side = Container::new(left_menu_content)
            .width(Length::Shrink)
            .height(Length::Fill)
            .align_x(Horizontal::Left)
            .center_y()
            .style(theme::Container::Black);
        let left_half = Column::new()
            .align_items(Alignment::Center)
            .spacing(5)
            .padding(0)
            .push(left_side);
        // End Left Row Widget

        // Start Right Row Widget
        let right_side = match &self.view {
            View::CheckTypeView => {
                let check_type_view_column = Column::new()
                    .push(
                        Row::new()
                            .push( Text::new("Check Type Id: "))
                            .push(
                                text_input("Check Type Id", &self.check_type_id,)
                                    .on_input(Message::CheckTypeIdChanged)
                                    .padding(5)
                            )
                            .push( Text::new("Check Type Name: "))
                            .push(
                                text_input("Check Type Name", &self.check_type_name)
                                    .on_input(Message::CheckTypeNameChanged)
                                    .padding(5)
                            )
                    )
                    .push(

                        Row::new()
                            .push(
                                checkbox("Default Check Type:", self.default_check_type)
                                    .on_toggle(Message::DefaultCheckTypeChanged)
                            )
                            .push(
                                checkbox("Active Check Type:", self.active_check_type)
                                    .on_toggle(Message::ActiveCheckTypeChanged)
                            )
                    )
                    .padding(5).spacing(12);
                
                Container::new(check_type_view_column)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Right)
                .style(theme::Container::Black)
            }
            View::CheckView => {
                let check_view_column = Column::new()
                .push(
                    Row::new()
                        .push(Text::new("Check Id: "))
                        .push(
                            text_input("Check Id", &self.check_id)
                                .on_input(Message::CheckIdChanged)
                                .padding(5)
                        )
                        .push(Text::new("Check Name: "))
                        .push(
                            text_input("Check Name", &self.check_name)
                                .on_input(Message::CheckNameChanged)
                                .padding(5)
                        )                   
                )
                .padding(5).spacing(12);
//                .push(
//                    Row::new()
//                );
            
                Container::new(check_view_column)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Right)
                .style(theme::Container::Black)
            }
            View::GratuityView => {
                let gratuity_view_column = Column::new()
                    .push(
                        Row::new()
                            .push(Text::new("Gratuity Id: "))
                            .push(
                                text_input("Gratuity Id", &self.gratuity_id)
                                    .on_input(Message::GratuityIdChanged)
                                    .padding(5)
                            )
                            .push(Text::new("Gratuity Name: "))
                            .push(
                                text_input("Gratuity Name", &self.gratuity_name)
                                    .on_input(Message::GratuityNameChanged)
                                    .padding(5)
                            )
                            .push(Text::new("Gratuity rate: "))  
                            .push(
                                text_input("Gratuity rate", &self.gratuity_rate)
                                    .on_input(Message::GratuityRateChanged)
                                    .padding(5)
                            )                 
                    )
                    .push(
                        Row::new()
                            .push(Text::new("Assigned Revenue Categories: "))  
                            .push(
                                text_input("Assigned Revenue Categories: ", &self.gratuity_revenue_categories)
                                    .on_input(Message::GratuityRevenueCategoriesChanged)
                                    .padding(5)
                            )     
                            .push(
                                checkbox("Taxed:", self.gratuity_taxed)
                                    .on_toggle(Message::GratuityTaxedChanged)
                            )
                    )
                    .padding(5).spacing(12);
                
                
                
                
                Container::new(gratuity_view_column)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Right)
                    .style(theme::Container::Black)                
            }
            View::ItemView => {
                self.item_view.view()
            }
/*             View::ItemView => {   // view prior to moving logic to another file
                let add_item_button = Button::new(Text::new("Add Item"));

                let add_item_view_column = Column::new()
                    .push(
                        Row::new()
                            .push(Text::new("Item Id: "))
                            .push(
                                text_input("Item Id", &self.item_id)
                                    .on_input(Message::ItemIdChanged)
                                    .padding(5)
                            )
                            .push(Text::new("Item Name: "))
                            .push(
                                text_input("Item Name", &self.item_name)
                                    .on_input(Message::ItemNameChanged)
                                    .padding(5)
                            )
                            .push(Text::new("Item Price: "))
                            .push(
                                text_input("Item Price", &self.item_price)
                                    .on_input(Message::ItemPriceChanged)
                                    .padding(5)
                            )
                            .push(Text::new("Tax Group: "))
                            .push(
                                text_input("Tax Group", &self.item_tax_group)
                                    .on_input(Message::ItemTaxGroupChanged)
                                    .padding(5)
                            )
                            .push(
                                add_item_button
                                    .on_press(Message::AddSystemItem(Item::new(self.item_id.parse::<usize>().unwrap_or(1), self.item_name.clone(), self.item_price.parse::<f64>().unwrap_or(0.0), self.item_tax_group.clone())))
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
                                        .on_press(Message::EditSystemItem(item.id.clone()))
                                        .style(theme::Button::MediaStart)
                                        .padding(5))
                                    .push(Button::new(Text::new("Delete"))
                                        .on_press(Message::DeleteSystemItem(item.id.clone()))
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
            } */
            View::RevenueCategoryView => {
                let revenue_category_view_column = Container::new(Text::new("Revenue Category View"))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Right)
                    .style(theme::Container::Black);

                revenue_category_view_column
            }
            View::ServiceChargeView => {
                let service_charge_view_column = Container::new(Text::new("Service Charge View"))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Right)
                    .style(theme::Container::Black);

                service_charge_view_column
            }
            View::TaxGroupView => {
                let tax_group_view_column = Container::new(Text::new("Tax Group View"))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Right)
                    .style(theme::Container::Black);

                tax_group_view_column
            }           
            View::TaxView => {
                let tax_view_column = Container::new(Text::new("Tax View"))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Right)
                    .style(theme::Container::Black);

                tax_view_column
            }
            View::DefaultView => {
                let default_view_column = Container::new(Text::new("Default View"))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Right)
                    .style(theme::Container::Black);

                default_view_column

            }
            _ => {
                let default_view_column = Container::new(Text::new("Default View"))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Right)
                    .style(theme::Container::Black);

                default_view_column
            }
        };

        let right_half = Column::new()
            .spacing(5)
            .padding(0)
            .push(right_side);
        // End Right Row Widget

        let main = row![left_half, right_half].spacing(10);

        Container::new(main)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(15)
        .into()

    }
}