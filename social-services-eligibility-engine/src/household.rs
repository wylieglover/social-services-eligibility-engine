use rust_decimal::Decimal;
use crate::member::Member;

#[derive(Debug)]
pub struct Household {
    pub members: Vec<Member>,
    pub total_asset_value: Decimal,
    pub state: String,
}

impl Household {
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
            total_asset_value: Decimal::ZERO,
            state: String::new(),
        }
    }

    pub fn add_member(&mut self, member: Member) {
        self.members.push(member);
    }

    pub fn size(&self) -> usize {
        self.members.len()
    }

    pub fn total_income(&self) -> Decimal {
        self.members.iter().map(|m| m.monthly_gross_income).sum()
    }
}