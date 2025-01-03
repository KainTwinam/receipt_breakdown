use iced::widget::{button, column, container, row, text, text_input, vertical_rule, Container, center, horizontal_space};
use iced::{Element, Length, Settings, Subscription, Theme, Task, Color, Vector};
mod ui;
use ui::ItemView;
mod items;
use items::Item;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::collections::BTreeMap;
use crate::ui::custom_appearances::{pos_table_header, pos_table_row};
use crate::data::window::{default_size, Error, MIN_SIZE};
mod window;
use window::Window;
mod data;


pub struct AppState {
    item_view: ItemView
}

pub enum WindowType {
    Main,
    ItemList,
    CreateItem
}

pub struct WindowState {
    id: window::Id,
    title: String,
    window_type: WindowType,
}

struct RC {
    current_view: Page,
    main_window: Window,
    windows: BTreeMap<window::Id, Window>,

}

#[derive(Debug, Clone)]
enum Message {
    UI(ui::Message),
    SpawnWindow(String),

    //testing multi Window application
    OpenWindow,
    WindowOpened(window::Id),
    WindowClosed(window::Id),
    TitleChanged(window::Id, String),
}


pub enum Page {
    Home(ui::ItemView),
    CreateItem,
}


#[derive(Default, Debug, Clone, Eq, PartialEq)]
struct Quantity(i32);

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Use an alternate format if the `#` flag is present
            write!(f, "Quantity: {}", self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
struct TaxGroup {
    id: i32,
    name: String,
    taxes: Vec<Tax>,
}

impl TaxGroup {
    fn new(id: i32, name: String, taxes: Vec<Tax>) -> Self {
//        println!("==Creating new Tax Group==");
//        println!("id: {}", id);
//        println!("name: {}", name);
        for tax in taxes.iter() {
//            println!("tax: {}", tax.name);
        };
//        println!("=========================");
//        println!("");
        
        TaxGroup {
            id: id,
            name: name,
            taxes: taxes
        }
    }
    
    fn get_tax_percent(&self, tax_name: &str) -> Option<f64> {
        // Search for the tax by name and return its percent if found
        for tax in &self.taxes {
            if tax.name == tax_name {
                return Some(tax.percent);
            }
        }
        None
    }

}

#[derive(Default, Debug, Clone, PartialEq)]
struct Tax {
    id: i32,
    percent: f64,
    name: String,
}

impl Tax {
    fn new(id: i32, percent: f64, name: String) -> Self {
        let tax_as_percentage = percent * 100.0;
//        println!("==Creating new Tax==");
//        println!("id: {}", id);
//        println!("name: {}", name);
//        println!("percent: {}%", tax_as_percentage);
//        println!("=========================");
//        println!("");
        
        Tax {
            id: id,
            percent: percent,
            name: name
        }
    }
}

impl Eq for Tax {}

impl Hash for Tax {
    fn hash<H: Hasher>(&self, state: &mut H){
        self.id.hash(state);
        self.name.hash(state);

        let percent_bits = self.percent.to_bits();
        percent_bits.hash(state);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct ServiceCharge {
    id: i32,
    percent: f64,
    name: String,
    tax_group: TaxGroup,
    tax_overide: bool,
    
}

impl ServiceCharge {
    fn new(id: i32, percent: f64, name: String, tax_group: TaxGroup, tax_overide: bool) -> Self {
        let tax_as_percentage = percent * 100.0;
//        println!("==Creating new Service Charge==");
//        println!("id: {}", id);
//        println!("name: {}", name);
//        println!("percent: {}%", tax_as_percentage);
//        println!("tax_group: {}", tax_group.name);
//        println!("tax_overide: {}", tax_overide.to_string());
//        println!("=========================");
//        println!("");
        
        ServiceCharge {
            id: id,
            percent: percent,
            name: name,
            tax_group: tax_group,
            tax_overide: tax_overide,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Gratuity {
    id: i32,
    percent: f64,
    name: String,
    tax_group: TaxGroup,
    tax_overide: bool,
}

impl Gratuity {
    fn new(id: i32, percent: f64, name: String, tax_group: TaxGroup, tax_overide: bool) -> Self {
        let tax_as_percentage = percent * 100.0;
//        println!("==Creating new Gratuity==");
//        println!("id: {}", id);
//        println!("name: {}", name);
//        println!("percent: {}%", tax_as_percentage);
//        println!("tax_group: {}", tax_group.name);
//        println!("tax_overide: {}", tax_overide.to_string());
//        println!("=========================");
//        println!("");
        
        Gratuity {
            id: id,
            percent: percent,
            name: name,
            tax_group: tax_group,
            tax_overide: tax_overide,
        }
    }
}

struct Totals {
    subtotal: f64,
    total: f64,
    tax: f64,
    service_charge: f64,
    gratuity: f64,
}


#[derive(Default, Debug, Clone, PartialEq)]
struct Receipt {
    id: i32,
    name: String,
    order: Vec<(Item, Quantity)>,
    service_charge: ServiceCharge,
    gratuity: Gratuity,
}

impl Receipt {
    fn new(id: i32, name: String, service_charge: ServiceCharge, gratuity: Gratuity) -> Self {
//        println!("==Creating new Receipt==");
//        println!("id: {}", id);
//        println!("name: {}", name);
//        println!("service charge: {}", service_charge.name);
//        println!("gratuity: {}", gratuity.name);
//        println!("=========================");
//        println!("");
    
        Receipt {
            id: id,
            name: name,
            order: Vec::new(),
            service_charge: service_charge,
            gratuity: gratuity,
        }
    }
    
    fn add_item(self: &mut Self, item: Item, quantity: Quantity){
        self.order.push((item.clone(), quantity.clone()));
//        println!("Adding {:?} {} to receipt {}", quantity.to_string(), item.name, self.name);
//        println!("");
    }
    
    fn print_receipt(self: Self){
        let calculation = self.calculate_total();
    
//        println!("==================================================");
//        println!("= Item Name  |  Item Price  |  Quantity  |  Cost =");
        for (item, quantity) in &self.order {
            let cost = item.price * quantity.0 as f64;
//            println!("= {}  |  {}  |  {}  |  {}", item.name, item.price, quantity, cost);
        }
//        println!("=                                                =");
//        println!("= Subtotal:         ${:.2}", calculation.subtotal);
//        println!("= Tax:              ${:.2}", calculation.tax);
//        println!("= Service Charge:   ${:.2}", calculation.service_charge);
//        println!("= Gratuity:         ${:.2}", calculation.gratuity);
//        println!("= Total:            ${:.2}", calculation.total);
//        println!("==================================================");
    }

/*     fn build_ui(self: &Self) -> Element<Message> {
        let calculation = self.calculate_total();

        for (item, quantity) in &self.order {
            let cost = item.price * quantity.0 as f64;
            println!("= {}  |  {}  |  {}  |  {}", item.name, item.price, quantity, cost);
        }
    } */
    
    fn calculate_total(self: &Self) -> Totals {
        let mut total = 0.0;
        let mut tax = 0.0;
        let mut service = 0.0;
        let mut grat = 0.0;
        let mut subtotal = 0.0;

        for (item, quantity) in &self.order {
            if item.tax_overide {
                // Add price directly if tax is overridden
                total += item.price * quantity.0 as f64;
//                println!("Adding {} * {} to total.", item.price, quantity.0);
            } else {
                // Calculate tax from the tax group
                if let Some(tax_percent) = item.tax_group.get_tax_percent("Sales Tax") {
                    let tax_amount = item.price * tax_percent;
                    subtotal += item.price * quantity.0 as f64;
                    total += (item.price + tax_amount) * quantity.0 as f64;
                    tax += tax_amount * quantity.0 as f64;
//                    println!(
//                        "Adding {} with tax {:.2} * {:.2} to total.",
//                        item.price, tax_amount, quantity.0
//                    );
                } else {
//                    println!("No applicable tax found for item {}", item.name);
                }
            }
        }
        
        let service_charge_amount = self.service_charge.percent * total;
        let gratuity_amount = self.gratuity.percent * total;
        service = service_charge_amount.clone();
        grat = gratuity_amount.clone();
        
//        println!("Adding service charge amount: {:.2}", service_charge_amount.clone());
//        println!("Adding gratuity amount: {:.2}", gratuity_amount.clone());
        
        total += service_charge_amount;
        total += gratuity_amount;

//        println!("Final Total: {:.2}", total);
//        println!("");
        
        Totals {
            subtotal: subtotal,
            total: total,
            tax: tax,
            service_charge: service,
            gratuity: grat,
        }
    }

/*     // Generate all possibilities
    fn calculate_all_possibilities(&self) -> Vec<f64> {
        let item_count = self.items.len();

        // All combinations for item taxation: 2^item_count possibilities
        let item_tax_combinations = (0..2_usize.pow(item_count as u32))
            .map(|x| (0..item_count).map(|i| (x >> i) & 1 == 1).collect::<Vec<bool>>());

        let mut results = Vec::new();

        for items_taxed in item_tax_combinations {
            for sc_taxed in [true, false] {
                for gr_taxed in [true, false] {
                    let total = self.calculate_total(items_taxed.clone(), sc_taxed, gr_taxed);
                    results.push(total);
                }
            }
        }
        results
    } */
}

impl RC {
    fn new() -> (Self, Task<Message>) {
        let item_view_state = ItemView::new();

        let (_id, open) = window::open(window::Settings::default());
        let (main_window_id, open_main_window) = window::open(window::Settings {
            size: window::default_size(),
            position: window::Position::Default,
            min_size: Some(window::MIN_SIZE),
            exit_on_close_request: false,
            ..window::settings()
        });

        let main_window = Window::new(main_window_id, "Main Window".to_string());

        (
            Self {
                windows: BTreeMap::new(),
                main_window: main_window,
                current_view: Page::Home(item_view_state),
            },
            open.map(Message::WindowOpened),
        )
    }


    fn update(state: &mut Self, message: Message) -> Task<Message> {
        match message {
            Message::UI(ui_message) => {
                match &mut state.current_view {
                    Page::Home(item_view_state) => {
                        ItemView::update(item_view_state, ui_message).map(Message::UI)
                    }
                    Page::CreateItem => {
                        Task::none()
                    }
                }
            }
            Message::SpawnWindow(window_title) => {
                let (window_id, window) = window::open(window::Settings {
                    size: window::default_size(),
                    position: window::Position::Default,
                    min_size: Some(window::MIN_SIZE),
                    exit_on_close_request: false,
                    ..window::settings()
                });

                let new_window = Window::new(window_id.clone(), window_title);

                state.windows.insert(window_id, new_window);
                Task::none()
            }
            Message::OpenWindow => {
                let Some(last_window) = state.windows.keys().last() else {
                    return Task::none();
                };

                iced::window::get_position(*last_window)
                    .then(|last_position| {
                        let position = last_position.map_or(
                            window::Position::Default,
                            |last_position| {
                                window::Position::Specific(
                                    last_position + Vector::new(20.0, 20.0),
                                )
                            },
                        );

                        let (_id, open) = window::open(window::Settings {
                            position,
                            ..window::Settings::default()
                        });

                        open
                    })
                    .map(Message::WindowOpened)
            }
            Message::WindowOpened(id) => {
                let window = Window::new(id, "It's a new window!".to_string());
                let focus_input = text_input::focus(format!("input-{id}"));

                state.windows.insert(id, window);

                focus_input
            }
            Message::WindowClosed(id) => {
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

        let item_view_state = ItemView::new();

        if let Some(window) = state.windows.get(&window_id) {

            if window.id == state.main_window.id {
                column![
                    ItemView::view(&item_view_state).map(Message::UI),
                    button(text("New Window")).on_press(Message::OpenWindow),
                ].into()
            } else {
                column![
                    text!("Do it again, but better"),
                    button(text("New Window")).on_press(Message::OpenWindow),
                ].into()
            }
/*             match &state.current_view {
                Page::Home(item_view_state) => {
                    column![
                        ItemView::view(&item_view_state).map(Message::UI),
                        button(text("New Window")).on_press(Message::OpenWindow),
                    ].into()
                }
                Page::CreateItem => {
                    column![
                        text!("Do it again, but better"),
                        button(text("New Window")).on_press(Message::OpenWindow),
                    ].into()
                }
            } */
                
            

        }    
        else {
            horizontal_space().into()
        }
    }

    fn theme(state: &Self, window_id: window::Id) -> Theme {
        Theme::Dracula
    }

}

fn main() -> iced::Result {
    
    iced::daemon("Receipt Calculator", RC::update, RC::view)
        .theme(RC::theme)
        .run_with(RC::new)
}

