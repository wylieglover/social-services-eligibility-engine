use social_services_eligibility_engine::eligibility::EligibilityProgram;
use social_services_eligibility_engine::household::Household;
use social_services_eligibility_engine::member::{Member, EmploymentStatus};
use social_services_eligibility_engine::decision::Decision;
use social_services_eligibility_engine::snap::SnapProgram;
use rust_decimal_macros::dec;

#[test]
fn test_clearly_eligible() {
    let mut household = Household::new();
    household.add_member(Member::new("Wylie".to_string(), 27, EmploymentStatus::FullTime, dec!(800), Some(40), Some(true)));
    household.add_member(Member::new("Franchette".to_string(), 28, EmploymentStatus::Unemployed, dec!(800), Some(0), Some(true)));

    let program = SnapProgram;
    let result = program.eligibility(&household);

    if let Decision::Eligible { total_household_income, number_of_members } = result {
        assert_eq!(total_household_income, dec!(1600));
        assert_eq!(number_of_members, 2);
    } else {
        panic!("Expected Eligible but got Ineligible");
    }
}

#[test]
fn test_clearly_ineligible() {
    let mut household = Household::new();
    household.add_member(Member::new("Wylie".to_string(), 27, EmploymentStatus::FullTime, dec!(4000), Some(40), Some(true)));
    household.add_member(Member::new("Franchette".to_string(), 28, EmploymentStatus::FullTime, dec!(8000), Some(40), Some(true)));

    let program = SnapProgram;
    let result = program.eligibility(&household);

    if let Decision::Ineligible { total_household_income, contributing_members, threshold } = result {
        assert_eq!(total_household_income, dec!(12000));
        assert_eq!(contributing_members.len(), 2);
        assert_eq!(threshold, dec!(2292));
    } else {
        panic!("Expected Ineligible but got eligible");
    }
}

#[test]
fn test_boundary() {
    let mut household = Household::new();
    household.add_member(Member::new("Wylie".to_string(), 27, EmploymentStatus::FullTime, dec!(1146), Some(40), Some(true)));
    household.add_member(Member::new("Franchette".to_string(), 28, EmploymentStatus::FullTime, dec!(1146), Some(40), Some(true)));

    let program = SnapProgram;
    let result = program.eligibility(&household);

    if let Decision::Eligible { total_household_income, number_of_members } = result {
        assert_eq!(total_household_income, dec!(2292));
        assert_eq!(number_of_members, 2);
    } else {
        panic!("Expected Eligible but got Ineligible");
    }
}

#[test]
fn test_large_household() {
    let mut household = Household::new();
    household.add_member(Member::new("Wylie".to_string(), 27, EmploymentStatus::Unemployed, dec!(0), Some(0), Some(true)));
    household.add_member(Member::new("Franchette".to_string(), 28, EmploymentStatus::FullTime, dec!(3800), Some(40), Some(true)));
    household.add_member(Member::new("Bob".to_string(), 30, EmploymentStatus::Contractor, dec!(1600), Some(30), Some(true)));
    household.add_member(Member::new("Frank".to_string(), 32, EmploymentStatus::PartTime, dec!(1200), Some(25), Some(true)));
    household.add_member(Member::new("Bill".to_string(), 24, EmploymentStatus::Unemployed, dec!(0), Some(0), Some(true)));
    household.add_member(Member::new("Joe".to_string(), 23, EmploymentStatus::PartTime, dec!(800), Some(0), Some(true)));
    household.add_member(Member::new("Ryan".to_string(), 32, EmploymentStatus::FullTime, dec!(6000), Some(40), Some(true)));
    household.add_member(Member::new("Richard".to_string(), 28, EmploymentStatus::SelfEmployed, dec!(400), Some(40), Some(true)));
    household.add_member(Member::new("Elijah".to_string(), 28, EmploymentStatus::SelfEmployed, dec!(3000), Some(40), Some(true)));

    let program = SnapProgram;
    let result = program.eligibility(&household);

    if let Decision::Ineligible { total_household_income, contributing_members, threshold } = result {
        assert_eq!(total_household_income, dec!(16800));
        assert_eq!(contributing_members.len(), 7);
        assert_eq!(threshold, dec!(6463));
    } else {
        panic!("Expected Ineligible but got Eligible");
    }
}
