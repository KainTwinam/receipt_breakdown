use crate::view::views::View;
use crate::types::type_definitions::Type;

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

}