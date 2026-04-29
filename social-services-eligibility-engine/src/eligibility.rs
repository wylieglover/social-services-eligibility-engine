use crate::household::Household;
use crate::decision::Decision;

pub trait EligibilityProgram {
    fn eligibility(&self, household: &Household) -> Decision;
}