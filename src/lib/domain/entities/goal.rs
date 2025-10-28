use crate::domain::entities::{
    GoalId,
    Quantity,
    Year,
    Quarter,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Goal {
    pub id: GoalId,
    pub name: String,
    pub metric: String,
    pub target: Quantity,
    pub current: Quantity,
    pub year: Year,
    pub quarter: Quarter,
    pub rationale: String,
}
