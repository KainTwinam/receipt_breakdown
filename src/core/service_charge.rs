use crate::core::tax_group::TaxGroup;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ServiceCharge {
    id: i32,
    pub percent: f64,
    name: String,
    tax_group: TaxGroup,
    tax_overide: bool,
    
}

impl ServiceCharge {
    pub fn new(id: i32, percent: f64, name: String, tax_group: TaxGroup, tax_overide: bool) -> Self {
        let tax_as_percentage = percent * 100.0;

        
        ServiceCharge {
            id: id,
            percent: percent,
            name: name,
            tax_group: tax_group,
            tax_overide: tax_overide,
        }
    }
}