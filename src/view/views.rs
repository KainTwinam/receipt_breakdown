#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum View {
    DefaultView,
    CheckTypeView,
    CheckView,
    GratuityView,
    ItemView,
    RevenueCategoryView,
    ServiceChargeView,
    TaxGroupView,
    TaxView,
}