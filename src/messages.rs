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

    // item messages, these are the same as the item_view messages while I migrate to seperating the view system
    ItemView(ItemViewMessage), // this is the message that will actually update the view in the main app after migration.
/*     ItemIdChanged(String),
    ItemNameChanged(String),
    ItemPriceChanged(String),
    ItemTaxGroupChanged(String),
    AddSystemItem(Item), // button to add new items to the system
    DeleteSystemItem(u64),
    EditSystemItem(u64), */

}

#[derive(Debug, Clone)]
pub enum ItemViewMessage {
    ItemIdChanged(String),
    ItemNameChanged(String),
    ItemPriceChanged(String),
    ItemTaxGroupChanged(String),
    AddSystemItem(Item), // button to add new items to the system
    DeleteSystemItem(u64),
    EditSystemItem(u64),
}