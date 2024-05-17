use crate::types::type_definitions::{CheckType, Check, Gratuity, Item, RevenueCategory, ServiceCharge, TaxGroup, Tax, };
use crate::types::type_definitions::Type;
use crate::view::views::View;


#[derive(Debug, Clone)]
pub enum Message {
    UpdateView(Type),

    CheckTypeIdChanged(String),
    CheckTypeNameChanged(String),
    CheckTypeDescriptionChanged(String),
    DefaultCheckTypeChanged(bool),
    ActiveCheckTypeChanged(bool),

    CheckIdChanged(String),
    CheckNameChanged(String),

    GratuityIdChanged(String),
    GratuityNameChanged(String),
    GratuityRateChanged(String),
    GratuityRevenueCategoriesChanged(String),
    GratuityTaxedChanged(bool),

    ItemIdChanged(String),
    ItemNameChanged(String),
    ItemPriceChanged(String),
    ItemTaxGroupChanged(String),
    AddSystemItem(Item), // button to add new items to the system
    DeleteSystemItem(usize),
    EditSystemItem(usize),

}