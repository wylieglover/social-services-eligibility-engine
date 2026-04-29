use social_services_eligibility_engine::{
    household::Household,
    member::{Member, EmploymentStatus},
    eligibility::EligibilityProgram,
    snap::SnapProgram,
};
use rust_decimal_macros::dec;

fn main() {
    let mut household = Household::new();
    household.add_member(Member::new("Wylie".to_string(), 27, EmploymentStatus::Unemployed, dec!(0), Some(0), Some(true)));
    household.add_member(Member::new("Franchette".to_string(), 28, EmploymentStatus::FullTime, dec!(1000.00), Some(40), Some(true)));

    let program = SnapProgram;
    let is_eligible = program.eligibility(&household);
    
    println!("{:?}", is_eligible);
}
