use rust_decimal::Decimal;

#[derive(Debug)]
pub struct MemberSummary {
    pub name: String,
    pub monthly_income: Decimal
}

#[derive(Debug)]
pub enum Decision {
    Eligible {
        total_household_income: Decimal,
        number_of_members: usize
    },
    Ineligible {
        total_household_income: Decimal,
        contributing_members: Vec<MemberSummary>,
        threshold: Decimal
    }
}