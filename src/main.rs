use iced::widget::{button, column, text, text_input, horizontal_space, row};
use iced::{Element, Subscription, Theme, Task, Vector};
use iced_core::alignment::Vertical;
mod core;
mod ui;
use ui::{
    item_list_view,
    add_item_view,
};
use ui::{
    item_list_view::ItemView,
    add_item_view::AddItemForm
};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::RefCell;
mod window;
mod data;

#[derive(Debug)]
pub struct AppState {
    item_view: ItemView,
    add_item_view: AddItemForm,
}

#[derive(Debug, Clone, Copy)]
pub enum WindowType {
    Main,
    ItemList,
    CreateItem
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
    ItemList(item_list_view::Message),
    AddItem(add_item_view::Message),

    //testing multi Window application
    MainWindowOpened(window::Id),
    OpenWindow(WindowType, String),
    WindowOpened(WindowState),
    WindowClosed(window::Id),
    TitleChanged(window::Id, String),
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
            item_view: ItemView::new(),
            add_item_view: AddItemForm::new(),
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

                //ItemView::update(&mut app_state.add_item_view, ui_message).map(Message::AddItem)
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
                    let item_view = if let Ok(app_state) = state.shared_state.try_borrow() {
                        app_state.item_view.clone()
                    } else {
                        ItemView::new()
                    };

                    column![
                        row![
                            text("Items").size(25),
                            
                        ],                        
                        ItemView::view(&item_view).map(Message::ItemList),
                        row![
                            button(text("Create Items")).on_press(Message::OpenWindow(WindowType::CreateItem, "Add Items".to_string())),
                        ].align_y(Vertical::Bottom),
                        
                        
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
                    column![
                        text!("Item List"),
                        button(text("New Window")).on_press(Message::OpenWindow(WindowType::ItemList, "Another Window".to_string())),
                    ].into()
                }
            }
        }    
        else {
            horizontal_space().into()
        }
    }

    fn theme(state: &Self, window_id: window::Id) -> Theme {
        Theme::Dracula
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

