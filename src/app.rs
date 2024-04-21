use crate::messages::Message;
use crate::types::type_definitions::Type;
use crate::view::views::View;
use crate::theme;
use crate::components::theme::themes::Themes;
use crate::widget::{Column, Container, Button, Element, Row, Text };

use iced::executor;
use iced::widget::{ checkbox, row, scrollable, text_input,};
use iced::{Application, Command, Length,};
use iced::alignment::{Alignment, Horizontal};

pub struct ReceiptCalculator {
    pub view: View,
    pub theme: Themes,
    pub types: Vec<Type>,

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
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Receipt Breakdown Calculator")
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
                let item_view_column = Container::new(Text::new("Item View"))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Right)
                    .style(theme::Container::Black);

                item_view_column
            }
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