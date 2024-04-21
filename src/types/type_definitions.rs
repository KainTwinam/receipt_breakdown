#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    CheckType,
    Check,
    Gratuity,
    Item,
    RevenueCategory,
    ServiceCharge,
    TaxGroup,
    Tax,
}

#[derive(Debug, PartialEq,)]
pub struct CheckType {
    id: i32,
    name: String,
    description: String,
    tax_groups: Vec<TaxGroup>,
    gratuities: Vec<Gratuity>,
    service_charges: Vec<ServiceCharge>,
    is_default: bool,
    is_active: bool,
}

impl CheckType{
    pub fn new(id: i32, name: String, description: String, tax_groups: Vec<TaxGroup>, gratuities: Vec<Gratuity>, service_charges: Vec<ServiceCharge>, is_default: bool, is_active: bool) -> Self {
        CheckType {
            id,
            name,
            description,
            tax_groups,
            gratuities,
            service_charges,
            is_default,
            is_active,
        }
    }
}

#[derive(Debug, PartialEq,)]
pub struct Check {
    id: i32,
    name: String,
    check_type: CheckType,
    items: Vec<Item>,
}

impl Check{
    pub fn new(id: i32, name: String, check_type: CheckType, items: Vec<Item>) -> Self {
        Check {
            id,
            name,
            check_type,
            items,
        }
    }
}

#[derive(Debug, PartialEq,)]
pub struct Gratuity {
    id: i32,
    name: String,
    rate: f64,
    revenue_categories: Vec<RevenueCategory>,
    taxed: bool,
}

impl Gratuity{
    pub fn new(id: i32, name: String, rate: f64, revenue_categories: Vec<RevenueCategory>, taxed: bool) -> Self {
        Gratuity {
            id,
            name,
            rate,
            revenue_categories,
            taxed,
        }
    }
}

#[derive(Debug, PartialEq, )]
pub struct Item {
    id: i32,
    name: String,
    cost: f64,
    tax_group: TaxGroup,
}

impl Item{
    pub fn new(id: i32, name: String, cost: f64, tax_group: TaxGroup) -> Self {
        Item {
            id,
            name,
            cost,
            tax_group,
        }
    }
}

#[derive(Debug, PartialEq,)]
pub struct RevenueCategory {
    name: String,
}

impl RevenueCategory{
    pub fn new(name: String) -> Self {
        RevenueCategory {
            name,
        }
    }
}


#[derive(Debug, PartialEq,)]
pub struct ServiceCharge {
    id: i32,
    name: String,
    rate: f64,
    taxed: bool,
}

impl ServiceCharge{
    pub fn new(id: i32, name: String, rate: f64, taxed: bool) -> Self {
        ServiceCharge {
            id,
            name,
            rate,
            taxed,
        }
    }
}

#[derive(Debug, PartialEq, )]
pub struct TaxGroup {
    id: i32,
    name: String,
    taxes: Vec<Tax>,
}

impl TaxGroup{
    pub fn new(id: i32, name: String, taxes: Vec<Tax>) -> Self {
        TaxGroup {
            id,
            name,
            taxes,
        }
    }
}

#[derive(Debug, PartialEq,)]
pub struct Tax {
    id: i32,
    name: String,
    rate: f64,
}

impl Tax{
    pub fn new(id: i32, name: String, rate: f64) -> Self {
        Tax {
            id,
            name,
            rate,
        }
    }
}