use crate::core::tax_group::TaxGroup;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Gratuity {
    id: i32,
    pub percent: f64,
    name: String,
    tax_group: TaxGroup,
    tax_overide: bool,
}

impl Gratuity {
    pub fn new(id: i32, percent: f64, name: String, tax_group: TaxGroup, tax_overide: bool) -> Self {
        let tax_as_percentage = percent * 100.0;
        
        Gratuity {
            id: id,
            percent: percent,
            name: name,
            tax_group: tax_group,
            tax_overide: tax_overide,
        }
    }
}