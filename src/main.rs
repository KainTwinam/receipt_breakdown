use iced::widget::{button, column, horizontal_rule, horizontal_space, row, text, text_input, vertical_rule, Container};
use iced::{Element, Subscription, Theme, Task, Vector};
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

struct RC {
    main_window_id: window::Id,
    windows: BTreeMap<window::Id, WindowState>,
    shared_state: Rc<RefCell<AppState>>,
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

    //testing multi Window application
    MainWindowOpened(window::Id),
    OpenWindow(WindowType, String),
    WindowOpened(WindowState),
    WindowClosed(window::Id),
    TitleChanged(window::Id, String),

    //ui test
    TestingUIStuff(core::testing_ui_stuff::Message),
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

                let focus_input = text_input::focus(format!("input-{id}"));

                state.windows.insert(id, window);

                focus_input
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
        }
    }


    fn view(state: &Self, window_id: window::Id) -> Element<Message> {

        if let Some(window) = state.windows.get(&window_id) {

            match window.window_type {
                WindowType::Main => {
                    row![
                        column![
                            button(text("Items")).on_press(Message::OpenWindow(WindowType::ItemList, "Item List".to_string())).width(iced::Length::Fill).style(button::primary), 
                            button(text("Tax Groups")).on_press(Message::OpenWindow(WindowType::TaxGroupList, "Tax Group List".to_string())).width(iced::Length::Fill).style(button::primary), 
                            button(text("Taxes")).on_press(Message::OpenWindow(WindowType::TaxList, "Tax List".to_string())).width(iced::Length::Fill).style(button::primary), 
                        ].width(100),
                        vertical_rule(2)
                    ].into()

/*                     let test_view = if let Ok(app_state) = state.shared_state.try_borrow() {
                            app_state.test_ui.clone()
                    } else {
                            core::testing_ui_stuff::TestView::new()
                    }; */

//old main view:
/* 
                    column![
//                         column![
//                            core::testing_ui_stuff::TestView::view(&test_view).map(Message::TestingUIStuff),


                        column![
                            row![
                                text("Items").size(25),
                            ],
                            row![
                                button(text("Item List")).on_press(Message::OpenWindow(WindowType::ItemList, "Item List".to_string())),
                                button(text("Create Items")).on_press(Message::OpenWindow(WindowType::CreateItem, "Add Items".to_string())),
                            ].spacing(5),
                        ].spacing(5),

                        iced::widget::Space::with_height(10),

                        column![
                            row![
                                text("Tax Groups").size(25),
                            ],
                            row![
                                button(text("Tax Group List")).on_press(Message::OpenWindow(WindowType::TaxGroupList, "Tax Group List".to_string())),
                                button(text("Create Tax Groups")).on_press(Message::OpenWindow(WindowType::CreateTaxGroup, "Create A Tax Group".to_string())),
                            ].spacing(5),
                        ].spacing(5),

                        iced::widget::Space::with_height(10),

                        column![
                            row![
                                text("Taxes").size(25),
                            ],
                            row![
                                button(text("Tax List")).on_press(Message::OpenWindow(WindowType::TaxList, "Tax List".to_string())),
                                button(text("Create Tax")).on_press(Message::OpenWindow(WindowType::CreateTax, "Create Tax".to_string())),
                            ].spacing(5),
                        ].spacing(5),

                    ].into() */
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

    fn theme(state: &Self, window_id: window::Id) -> Theme {
        Theme::CatppuccinFrappe
    }

    fn subscription(state: &Self) -> Subscription<Message> {
        iced::window::close_events().map(Message::WindowClosed)
    }


}



fn main() -> iced::Result {
    
    iced::daemon("Receipt Calculator", RC::update, RC::view)
        .subscription(RC::subscription)
        .theme(RC::theme)
        .run_with(RC::new)
}

