use iced::widget::{button, column, container, row, text, text_input, vertical_rule};
use iced::{Element, Length, Settings, Subscription, Theme, Task};
mod ui;
use ui::main_page;

#[derive(Default)]
struct RC {
    current_view: Page
}

#[derive(Debug, Clone)]
enum Message {

}

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default] Home,
}

#[derive(Clone)]
struct Item {
    name: String,
    price: f64,
    quantity: u32,
    tax_override: bool, // true means no tax applies to this item
}

impl Item {
    fn new() -> Self {
        Self {
            name: String::new(),
            price: 0.0,
            quantity: 0,
            tax_override: false,
        }
    }
}

#[derive(Default, Clone)]
struct Receipt {
    items: Vec<Item>,
    service_charge: f64,
    gratuity: f64,
    service_charge_taxable: bool,
    gratuity_taxable: bool,
    tax_rate: f64, // Tax rate as a percentage (e.g., 0.10 for 10%)
}

impl Receipt {
        fn new() -> Self {
            Self{
                items: Vec::new(),
                service_charge: 0.0,
                gratuity: 0.0,
                service_charge_taxable: false,
                gratuity_taxable: false,
                tax_rate: 0.08,

            }
        }
    
        // Calculate total cost for one possibility
        fn calculate_total(&self, items_taxed: Vec<bool>, sc_taxed: bool, gr_taxed: bool) -> f64 {
            let mut subtotal = 0.0;
            let mut tax_total = 0.0;
    
            // Item-wise calculations
            for (item, taxed) in self.items.iter().zip(items_taxed.iter()) {
                let item_cost = item.price * item.quantity as f64;
                subtotal += item_cost;
    
                if *taxed && !item.tax_override {
                    tax_total += item_cost * self.tax_rate;
                }
            }
    
            // Service charge
            subtotal += self.service_charge;
            if sc_taxed {
                tax_total += self.service_charge * self.tax_rate;
            }
    
            // Gratuity
            subtotal += self.gratuity;
            if gr_taxed {
                tax_total += self.gratuity * self.tax_rate;
            }
    
            subtotal + tax_total
        }
    
            // Generate all possibilities
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
            }
}

impl RC {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                current_view: Page::Home,
            },
            Task::none(),
        )
    }


    fn update(state: &mut Self, message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(state: &Self) -> Element<Message> {
        column![
            main_page(),
        ].into()
    }

 


    fn theme(state: &Self) -> Theme {
        Theme::Dracula
    }
}

fn main() -> iced::Result {
    
    iced::application("Receipt Calculator", RC::update, RC::view)
        .theme(RC::theme)
        .run_with(RC::new)
}

