use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Member {
    pub name: String,
    pub age: u8,
    pub employment_status: EmploymentStatus,
    pub monthly_gross_income: Decimal,
    pub hours_per_week: Option<u8>,
    pub is_able_bodied: Option<bool>
}

impl Member {
    pub fn new(name: String, age: u8, employment_status: EmploymentStatus, monthly_gross_income: Decimal, hours_per_week: Option<u8>, is_able_bodied: Option<bool>) -> Self {
        Self { name, age, employment_status, monthly_gross_income, hours_per_week, is_able_bodied }
    }

    pub fn has_income(&self) -> bool {
        self.monthly_gross_income != Decimal::ZERO
    }
}

#[derive(Debug)]
pub enum EmploymentStatus {
    FullTime,
    PartTime,
    Seasonal,
    Contractor,
    Temporary,
    SelfEmployed,
    Unemployed,
}