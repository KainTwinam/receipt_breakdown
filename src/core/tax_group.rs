use crate::core::tax::Tax;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaxGroup {
    id: i32,
    name: String,
    taxes: Vec<Tax>,
}

impl TaxGroup {
    pub fn new(id: i32, name: String, taxes: Vec<Tax>) -> Self {

        for tax in taxes.iter() {
//            println!("tax: {}", tax.name);
        };

        
        TaxGroup {
            id: id,
            name: name,
            taxes: taxes
        }
    }
    
    pub fn get_tax_percent(&self, tax_name: &str) -> Option<f64> {
        // Search for the tax by name and return its percent if found
        for tax in &self.taxes {
            if tax.name == tax_name {
                return Some(tax.percent);
            }
        }
        None
    }

}