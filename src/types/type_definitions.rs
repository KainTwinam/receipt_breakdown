use std::sync::atomic::{AtomicU64, Ordering};

static CHECKTYPE_ID_COUNTER: AtomicU64 = AtomicU64::new(0);
static CHECK_ID_COUNTER: AtomicU64 = AtomicU64::new(0);
static GRATUITY_ID_COUNTER: AtomicU64 = AtomicU64::new(0);
static ITEM_ID_COUNTER: AtomicU64 = AtomicU64::new(0);
static REVENUE_CATEGORY_ID_COUNTER: AtomicU64 = AtomicU64::new(0);
static SERVICE_CHARGE_ID_COUNTER: AtomicU64 = AtomicU64::new(0);
static TAX_GROUP_ID_COUNTER: AtomicU64 = AtomicU64::new(0);
static TAX_ID_COUNTER: AtomicU64 = AtomicU64::new(0);


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

#[derive(Debug, PartialEq, Clone,)]
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

#[derive(Debug, PartialEq, Clone,)]
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

#[derive(Debug, PartialEq, Clone,)]
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

#[derive(Debug, PartialEq, Clone,)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub price: f64,
    pub tax_group: String,
}

impl Item{
    pub fn new(id: u64, name: String, price: f64, tax_group: String) -> Self {
        let id = ITEM_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;

        Item {
            id,
            name,
            price,
            tax_group,
        }
    }
}

#[derive(Debug, PartialEq, Clone,)]
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

#[derive(Debug, PartialEq, Clone,)]
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

#[derive(Debug, PartialEq, Clone,)]
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


#[derive(Debug, PartialEq, Clone,)]
pub struct Tax {
    id: i32,
    name: String,
    rate: f64,
}

impl Tax{
    pub fn new(id: i32, name: &str, rate: f64) -> Self {
        Tax {
            id,
            name: String::from(name),
            rate,
        }
    }
}

