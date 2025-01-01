use iced::widget::{button, column, container, row, text, text_input, vertical_rule};
use iced::{Element, Length, Settings, Subscription, Theme, Task};
use crate::Receipt;
use crate::Item;
use crate::Message;

pub fn main_page() -> Element<'static, Message> {
    let receipt = Receipt::new();
    let item = Item::new();

    let row1 = row![
        text_input("Item Name", &item.name),

    ].into();

    row1
}


/* #[derive(Default, Clone)]
struct Receipt {
    items: Vec<Item>,
    service_charge: f64,
    gratuity: f64,
    service_charge_taxable: bool,
    gratuity_taxable: bool,
    tax_rate: f64, // Tax rate as a percentage (e.g., 0.10 for 10%)
} 

#[derive(Clone)]
struct Item {
    name: String,
    price: f64,
    quantity: u32,
    tax_override: bool, // true means no tax applies to this item
}

*/