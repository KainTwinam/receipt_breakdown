use crate::core::{
    items::Item,
    quantity::Quantity,
    service_charge::ServiceCharge,
    gratuity::Gratuity,
    total::Total,
};


#[derive(Default, Debug, Clone, PartialEq)]
pub struct Receipt {
    id: i32,
    name: String,
    order: Vec<(Item, Quantity)>,
    service_charge: ServiceCharge,
    gratuity: Gratuity,
}

impl Receipt {
    pub fn new(id: i32, name: String, service_charge: ServiceCharge, gratuity: Gratuity) -> Self {
    
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

    }
    
    fn print_receipt(self: Self){
        let calculation = self.calculate_total();

        for (item, quantity) in &self.order {
            let cost = item.price * quantity.0 as f64;

        }

    }
    
    fn calculate_total(self: &Self) -> Total {
        let mut total = 0.0;
        let mut tax = 0.0;
        let mut service = 0.0;
        let mut grat = 0.0;
        let mut subtotal = 0.0;

        for (item, quantity) in &self.order {
            if item.tax_overide {
                // Add price directly if tax is overridden
                total += item.price * quantity.0 as f64;
            } else {
                // Calculate tax from the tax group
                if let Some(tax_percent) = item.tax_group.get_tax_percent("Sales Tax") {
                    let tax_amount = item.price * tax_percent;
                    subtotal += item.price * quantity.0 as f64;
                    total += (item.price + tax_amount) * quantity.0 as f64;
                    tax += tax_amount * quantity.0 as f64;

                } else {

                }
            }
        }
        
        let service_charge_amount = self.service_charge.percent * total;
        let gratuity_amount = self.gratuity.percent * total;
        service = service_charge_amount.clone();
        grat = gratuity_amount.clone();
         
        total += service_charge_amount;
        total += gratuity_amount;
        
        Total {
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