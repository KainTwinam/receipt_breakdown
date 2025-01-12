use iced::widget::{button, column, horizontal_rule, horizontal_space, row, text, text_input, vertical_rule, Container};
use iced::{Alignment, Element, Subscription, Task, Theme, Vector};
use iced_core::alignment::Vertical;
mod core;
mod ui;
use ui::{
    //items
    item_list_view,
    add_item_view,
    //tax groups
    tax_group_list_view,
    add_tax_group_view,
    //taxes
    tax_list_view,
    add_tax_view,
    //gratuities
    gratuity_list_view,
    add_gratuity_view,
    //service charges
    service_charge_list_view,
    add_service_charge_view,
};
use ui::{
    //items
    item_list_view::ItemView,
    add_item_view::AddItemForm,
    //tax groups
    tax_group_list_view::TaxGroupView,
    add_tax_group_view::AddTaxGroupForm,
    //taxes
    tax_list_view::TaxView,
    add_tax_view::AddTaxForm,
    //gratuities
    gratuity_list_view::GratuityView,
    add_gratuity_view::AddGratuityForm,
    //service charges
    service_charge_list_view::ServiceChargeView,
    add_service_charge_view::AddServiceChargeForm,
};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::RefCell;
mod window;
mod data;

#[derive(Debug)]
pub struct AppState {
    //items
    item_view: ItemView,
    add_item_view: AddItemForm,

    //tax groups
    tax_group_view: TaxGroupView,
    add_tax_group_view: AddTaxGroupForm,

    //taxes
    tax_view: TaxView,
    add_tax_view: AddTaxForm,

    //gratuities
    gratuity_view: GratuityView,
    add_gratuity_view: AddGratuityForm,
    //service charges
    service_charge_view: ServiceChargeView,
    add_service_charge_view: AddServiceChargeForm,

    //test ui
    test_ui: core::testing_ui_stuff::TestView,

}

#[derive(Debug, Clone, Copy)]
pub enum WindowType {
    Main,

    //items
    ItemList,
    CreateItem,

    //tax groups
    TaxGroupList,
    CreateTaxGroup,

    //taxes
    TaxList,
    CreateTax,
}

#[derive(Debug, Clone)]
pub struct WindowState {
    id: window::Id,
    title: String,
    window_type: WindowType,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum View {
    #[default] Main,
    Items,
    TaxGroups,
    Taxes,
    Gratuities,
    ServiceCharges,
}

struct RC {
    main_window_id: window::Id,
    windows: BTreeMap<window::Id, WindowState>,
    shared_state: Rc<RefCell<AppState>>,
    view: View,
    window_size: (f32, f32),
}

#[derive(Debug, Clone)]
enum Message {
    //items
    ItemList(item_list_view::Message),
    AddItem(add_item_view::Message),

    //tax groups
    TaxGroupList(tax_group_list_view::Message),
    AddTaxGroup(add_tax_group_view::Message),

    //taxes
    TaxList(tax_list_view::Message),
    AddTax(add_tax_view::Message),

    //gratuities
    GratuityList(gratuity_list_view::Message),
    AddGratuity(add_gratuity_view::Message),

    //service charges
    ServiceChargeList(service_charge_list_view::Message),
    AddServiceCharge(add_service_charge_view::Message),

    //testing multi Window application
    MainWindowOpened(window::Id),
    OpenWindow(WindowType, String),
    WindowOpened(WindowState),
    WindowClosed(window::Id),
    TitleChanged(window::Id, String),

    //ui test
    TestingUIStuff(core::testing_ui_stuff::Message),

    //moving back to views inside the main window
    SetView(View),
    WindowResized((window::Id, iced::Size)),
    //KeyboardEvent((iced::keyboard::Key, iced::keyboard::Modifiers)),
    KeyboardEvent(iced::keyboard::Key),
}

impl RC {
    fn new() -> (Self, Task<Message>) {
        let (main_window_id, open_main_window) = window::open(window::Settings {
            size: window::default_size(),
            position: window::Position::Default,
            min_size: Some(window::MIN_SIZE),
            exit_on_close_request: true,
            ..window::settings()
        });

        let main_window = WindowState {
            id: main_window_id,
            title: "Receipt Calculator".to_string(),
            window_type: WindowType::Main,
        };

        let shared_state = Rc::new(RefCell::new(AppState{
            //items
            item_view: ItemView::new(),
            add_item_view: AddItemForm::new(),
            //tax groups
            tax_group_view: TaxGroupView::new(),
            add_tax_group_view: AddTaxGroupForm::new(),
            //taxes
            tax_view: TaxView::new(),
            add_tax_view: AddTaxForm::new(),

            //gratuities
            gratuity_view: GratuityView::new(),
            add_gratuity_view: AddGratuityForm::new(),
            //service charges
            service_charge_view: ServiceChargeView::new(),
            add_service_charge_view: AddServiceChargeForm::new(),

            //test ui
            test_ui: core::testing_ui_stuff::TestView::new(),

        }));

        let mut windows = BTreeMap::new();
        windows.insert(main_window_id, main_window);

        (
            Self {
                windows,
                main_window_id,
                shared_state,
                view: View::default(),
                window_size: (0_f32, 0_f32),
            },
            open_main_window.map(Message::MainWindowOpened),
        )
    }


    fn update(state: &mut Self, message: Message) -> Task<Message> {
        match message {
            Message::ItemList(ui_message) => {
                let mut app_state = state.shared_state.borrow_mut();
                ItemView::update(&mut app_state.item_view, ui_message).map(Message::ItemList)
            }
            Message::AddItem(msg) => {
                let mut app_state = state.shared_state.borrow_mut();

                if let Some(action) = add_item_view::AddItemForm::update( &mut app_state.add_item_view, msg ) {
                    match action {
                        add_item_view::Action::AddNewItem(item) => {
                            Task::perform(async move {
                                item_list_view::Message::NewItem(item)
                            }, Message::ItemList)
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::TaxGroupList(ui_message) => {
                let mut app_state = state.shared_state.borrow_mut();
                TaxGroupView::update(&mut app_state.tax_group_view, ui_message).map(Message::TaxGroupList)
            }
            Message::AddTaxGroup(msg) => {
                let mut app_state = state.shared_state.borrow_mut();

                if let Some(action) = add_tax_group_view::AddTaxGroupForm::update( &mut app_state.add_tax_group_view, msg ) {
                    match action {
                        add_tax_group_view::Action::AddNewTaxGroup(tax_group) => {
                            Task::perform(async move {
                                tax_group_list_view::Message::NewTaxGroup(tax_group)
                            }, Message::TaxGroupList)
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::TaxList(ui_message) => {
                let mut app_state = state.shared_state.borrow_mut();
                TaxView::update(&mut app_state.tax_view, ui_message).map(Message::TaxList)
            }
            Message::AddTax(msg) => {
                let mut app_state = state.shared_state.borrow_mut();

                if let Some(action) = add_tax_view::AddTaxForm::update( &mut app_state.add_tax_view, msg ) {
                    match action {
                        add_tax_view::Action::AddNewTax(tax) => {
                            Task::perform(async move {
                                tax_list_view::Message::NewTax(tax)
                            }, Message::TaxList)
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::GratuityList(ui_message) => {
                let mut app_state = state.shared_state.borrow_mut();
                GratuityView::update(&mut app_state.gratuity_view, ui_message).map(Message::GratuityList)
            }
            Message::AddGratuity(msg) => {
                let mut app_state = state.shared_state.borrow_mut();

                if let Some(action) = add_gratuity_view::AddGratuityForm::update( &mut app_state.add_gratuity_view, msg ) {
                    match action {
                        add_gratuity_view::Action::AddNewGratuity(gratuity) => {
                            Task::perform(async move {
                                gratuity_list_view::Message::NewGratuity(gratuity)
                            }, Message::GratuityList)
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::ServiceChargeList(ui_message) => {
                let mut app_state = state.shared_state.borrow_mut();
                ServiceChargeView::update(&mut app_state.service_charge_view, ui_message).map(Message::ServiceChargeList)
            }
            Message::AddServiceCharge(msg) => {
                let mut app_state = state.shared_state.borrow_mut();

                if let Some(action) = add_service_charge_view::AddServiceChargeForm::update( &mut app_state.add_service_charge_view, msg ) {
                    match action {
                        add_service_charge_view::Action::AddNewServiceCharge(service_charge) => {
                            Task::perform(async move {
                                service_charge_list_view::Message::NewServiceCharge(service_charge)
                            }, Message::ServiceChargeList)
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::TestingUIStuff(ui_message) => {
                let mut app_state = state.shared_state.borrow_mut();

                core::testing_ui_stuff::TestView::update(&mut app_state.test_ui, ui_message).map(Message::TestingUIStuff)
            }
            Message::OpenWindow(window_type, title) => {
                let Some(last_window) = state.windows.keys().last() else {
                    return Task::none();
                };

                let title_clone = title.clone(); // Clone here for the outer closure

                iced::window::get_position(*last_window)
                    .then(move |last_position| {
                        let position = last_position.map_or(
                            window::Position::Default,
                            |last_position| {
                                window::Position::Specific(
                                    last_position + Vector::new(20.0, 20.0),
                                )
                            },
                        );

                        let (id, open) = window::open(window::Settings {
                            size: window::default_size(),
                            position,
                            min_size: Some(window::MIN_SIZE),
                            exit_on_close_request: true,
                            ..window::settings()
                        });

                        let title_clone = title_clone.clone(); // Clone here for the outer closure

                        let new_window = WindowState {
                            id: id,
                            title: title_clone.to_owned(),
                            window_type: window_type
                        };

                        open.map(move |_| Message::WindowOpened(new_window.to_owned()))
                    })
            }
            Message::WindowOpened(new_window) => {
                let new_window = WindowState {
                    id: new_window.id,
                    title: new_window.title,
                    window_type: new_window.window_type,
                };

                let id_clone = new_window.id.clone();

                state.windows.insert(new_window.id, new_window);
                text_input::focus(format!("input-{}", id_clone))
            }
            Message::MainWindowOpened(id) => {

                let window = WindowState {
                    id: id,
                    title: "Receipt Calculator".to_string(),
                    window_type: WindowType::Main,
                };

                //let focus_input = text_input::focus(format!("input-{id}"));

                state.windows.insert(id, window);

                //focus_input
                Task::none()
            }
            Message::WindowClosed(id) => {
                println!("Window Closed Event Requested");
                state.windows.remove(&id);

                if state.windows.is_empty() {
                    iced::exit()
                } else {
                    Task::none()
                }
            }
            Message::TitleChanged(id, title) => {
                if let Some(window) = state.windows.get_mut(&id) {
                    window.title = title;
                }

                Task::none()
            }
            Message::SetView(view) => {
                state.view = view;
                Task::none()
            }
            Message::WindowResized((_id, size)) => {
                state.window_size = (size.width, size.height);

                Task::none()
            }
            Message::KeyboardEvent(key) => {
                
                Task::none()
            }
        }
    }


    fn view(state: &Self, window_id: window::Id) -> Element<Message> {

        if let Some(window) = state.windows.get(&window_id) {

            match window.window_type {
                WindowType::Main => {

                    let view: Element<'_, Message> = match state.view {
                        View::Main => {
                            column![
                                text("Message of the day:").size(25),
                                text("I have a message of the day, and it's placed here, on the main menu where it belongs."),
                            ].into()
                        }
                        View::Items => {
                            let item_view = if let Ok(app_state) = state.shared_state.try_borrow() {
                                app_state.item_view.clone()
                            } else {
                                ItemView::new()
                            };
        
                            let add_item_view = if let Ok(app_state) = state.shared_state.try_borrow() {
                                app_state.add_item_view.clone()
                            } else {
                                AddItemForm::new()
                            };
        
                            Container::new(
                                row![
                                    AddItemForm::view(&add_item_view).map(Message::AddItem),
                                    iced::widget::vertical_rule(1),        
                                    ItemView::view(&item_view).map(Message::ItemList),
                                ]
                            ).into()
                        }
                        View::TaxGroups => {
                            let tax_group_view = if let Ok(app_state) = state.shared_state.try_borrow() {
                                app_state.tax_group_view.clone()
                            } else {
                                TaxGroupView::new()
                            };

                            let add_tax_group = if let Ok(app_state) = state.shared_state.try_borrow() {
                                app_state.add_tax_group_view.clone()
                            } else {
                                AddTaxGroupForm::new()
                            };
        
                            Container::new(
                                row![
                                    AddTaxGroupForm::view(&add_tax_group).map(Message::AddTaxGroup),
                                    iced::widget::vertical_rule(1), 
                                    TaxGroupView::view(&tax_group_view).map(Message::TaxGroupList),
                                ]
                            ).into()
                        }
                        View::Taxes => {
                            let tax_view = if let Ok(app_state) = state.shared_state.try_borrow() {
                                app_state.tax_view.clone()
                            } else {
                                TaxView::new()
                            };

                            let add_tax = if let Ok(app_state) = state.shared_state.try_borrow() {
                                app_state.add_tax_view.clone()
                            } else {
                                AddTaxForm::new()
                            };
        
                            Container::new(
                                row![
                                    AddTaxForm::view(&add_tax).map(Message::AddTax),
                                    iced::widget::vertical_rule(1), 
                                    TaxView::view(&tax_view).map(Message::TaxList),
                                ]
                            ).into()
                        }
                        View::Gratuities => {
                            let gratuity_view = if let Ok(app_state) = state.shared_state.try_borrow() {
                                app_state.gratuity_view.clone()
                            } else {
                                GratuityView::new()
                            };

                            let add_gratuity = if let Ok(app_state) = state.shared_state.try_borrow() {
                                app_state.add_gratuity_view.clone()
                            } else {
                                AddGratuityForm::new()
                            };
        
                            Container::new(
                                row![
                                    AddGratuityForm::view(&add_gratuity).map(Message::AddGratuity),
                                    iced::widget::vertical_rule(1), 
                                    GratuityView::view(&gratuity_view).map(Message::GratuityList),
                                ]
                            ).into()
                        }
                        View::ServiceCharges => {
                            let service_charge_view = if let Ok(app_state) = state.shared_state.try_borrow() {
                                app_state.service_charge_view.clone()
                            } else {
                                ServiceChargeView::new()
                            };

                            let add_service_charge = if let Ok(app_state) = state.shared_state.try_borrow() {
                                app_state.add_service_charge_view.clone()
                            } else {
                                AddServiceChargeForm::new()
                            };
        
                            Container::new(
                                row![
                                    AddServiceChargeForm::view(&add_service_charge).map(Message::AddServiceCharge),
                                    iced::widget::vertical_rule(1), 
                                    ServiceChargeView::view(&service_charge_view).map(Message::ServiceChargeList),
                                ]
                            ).into()
                        }
                    };


                    let left_menu: Element<'_, Message> = row![
                        column![
                            text("Config").size(20),
                            button(text("Items")).on_press(Message::SetView(View::Items)).width(iced::Length::Fill).style(button::primary),
                            button(text("Tax Groups")).on_press(Message::SetView(View::TaxGroups)).width(iced::Length::Fill).style(button::primary),  
                            button(text("Taxes")).on_press(Message::SetView(View::Taxes)).width(iced::Length::Fill).style(button::primary),
                            button(text("Gratuities")).on_press(Message::SetView(View::Gratuities)).width(iced::Length::Fill).style(button::primary), 
                            button(text("Service Charges")).on_press(Message::SetView(View::ServiceCharges)).width(iced::Length::Fill).style(button::primary),  

                            text(format!("X: {}", state.window_size.0)),
                            text(format!("Y: {}", state.window_size.1)),

                        ].align_x(Alignment::Center).width(100),
                        vertical_rule(2)
                    ].into();

                    row![
                        left_menu,
                        Container::new(view),
                    ].into()

                }
                WindowType::CreateItem => {
                    let add_item_view = if let Ok(app_state) = state.shared_state.try_borrow() {
                        app_state.add_item_view.clone()
                    } else {
                        AddItemForm::new()
                    };

                    column![
                        AddItemForm::view(&add_item_view).map(Message::AddItem),
                    ].into()
                }
                WindowType::ItemList => {
                    let item_view = if let Ok(app_state) = state.shared_state.try_borrow() {
                        app_state.item_view.clone()
                    } else {
                        ItemView::new()
                    };

                    let add_item_view = if let Ok(app_state) = state.shared_state.try_borrow() {
                        app_state.add_item_view.clone()
                    } else {
                        AddItemForm::new()
                    };

                    Container::new(
                        row![
                            AddItemForm::view(&add_item_view).map(Message::AddItem),
                            iced::widget::vertical_rule(1),        
                            ItemView::view(&item_view).map(Message::ItemList),
                        ]
                    ).into()
                }
                WindowType::TaxGroupList => {
                    let tax_group_view = if let Ok(app_state) = state.shared_state.try_borrow() {
                        app_state.tax_group_view.clone()
                    } else {
                        TaxGroupView::new()
                    };

                    column![
                        row![text!("Tax Groups").size(25),],
                        TaxGroupView::view(&tax_group_view).map(Message::TaxGroupList),
                        row![
                            button(text("Create Tax Groups")).on_press(Message::OpenWindow(WindowType::CreateTaxGroup, "Create A Tax Group".to_string()))
                        ]
                    ].into()
                }
                WindowType::CreateTaxGroup => {
                    let add_tax_group = if let Ok(app_state) = state.shared_state.try_borrow() {
                        app_state.add_tax_group_view.clone()
                    } else {
                        AddTaxGroupForm::new()
                    };

                    column![
                        AddTaxGroupForm::view(&add_tax_group).map(Message::AddTaxGroup),
                    ].into()
                }
                WindowType::TaxList => {
                    let tax_view = if let Ok(app_state) = state.shared_state.try_borrow() {
                        app_state.tax_view.clone()
                    } else {
                        TaxView::new()
                    };

                    column![
                        row![text!("Taxes").size(25),],
                        TaxView::view(&tax_view).map(Message::TaxList),
                        row![
                            button(text("Create Tax")).on_press(Message::OpenWindow(WindowType::CreateTax, "Create Tax".to_string()))
                        ]
                    ].into()
                }
                WindowType::CreateTax => {
                    let add_tax = if let Ok(app_state) = state.shared_state.try_borrow() {
                        app_state.add_tax_view.clone()
                    } else {
                        AddTaxForm::new()
                    };

                    column![
                        AddTaxForm::view(&add_tax).map(Message::AddTax),
                    ].into()
                }
            }
        }    
        else {
            horizontal_space().into()
        }
    }


    fn theme(_state: &Self, _window_id: window::Id) -> Theme {
        Theme::CatppuccinFrappe
    }

    fn subscription(_state: &Self) -> Subscription<Message> {
        let window_closed = iced::window::close_events().map(Message::WindowClosed);
        let window_resized = iced::window::resize_events().map(Message::WindowResized);
        let keyboard_key_pressed = iced::keyboard::on_key_press(|key, _modifiers| Some(Message::KeyboardEvent(key)));

        Subscription::batch(vec![window_resized, window_closed, keyboard_key_pressed])
    }


}



fn main() -> iced::Result {
    
    iced::daemon("Receipt Calculator", RC::update, RC::view)
        .subscription(RC::subscription)
        .theme(RC::theme)
        .run_with(RC::new)
}

