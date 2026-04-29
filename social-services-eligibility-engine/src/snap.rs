use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::household::Household;
use crate::decision::{Decision, MemberSummary};
use crate::eligibility::EligibilityProgram;

pub struct SnapProgram;

impl EligibilityProgram for SnapProgram {
    fn eligibility(&self, household: &Household) -> Decision {
        let total_household_income = household.total_income();
        let number_of_members = household.size();
        let threshold = gross_income_limit(number_of_members);

        if total_household_income <= threshold {
            Decision::Eligible { total_household_income, number_of_members }
        } else {
            let contributing_members = household.members.iter()
                .filter(|m| m.has_income())
                .map(|m| { MemberSummary {
                    name: m.name.clone(),
                    monthly_income: m.monthly_gross_income
                }})
                .collect();

            Decision::Ineligible { total_household_income, contributing_members, threshold }
        }
    }
}

/// Returns the federal SNAP gross income limit for 2026
pub fn gross_income_limit(household_size: usize) -> Decimal {
    match household_size {
        1 => dec!(1696),
        2 => dec!(2292),
        3 => dec!(2888),
        4 => dec!(3483),
        5 => dec!(4079),
        6 => dec!(4675),
        7 => dec!(5271),
        8 => dec!(5867),
        n => dec!(5867) + dec!(596) * Decimal::from((n - 8) as u32),
    }
}