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

impl Goal {
    pub fn new<T: Into<String>>(
        name: T,
        metric: T,
        target: Quantity,
        current: Quantity,
        year: Year,
        quarter: Quarter,
        rationale: T,
    ) -> Self {
        Self {
            id: GoalId::new(),
            name: name.into(),
            metric: metric.into(),
            target,
            current,
            year,
            quarter,
            rationale: rationale.into(),
        }
    }

    pub fn update_current(&mut self, new_current: Quantity) {
        self.current = new_current;
    }

    pub fn update_target(&mut self, new_target: Quantity) {
        self.target = new_target;
    }

    pub fn update_name<T: Into<String>>(&mut self, new_name: T) {
        self.name = new_name.into();
    }

    pub fn update_metric<T: Into<String>>(&mut self, new_metric: T) {
        self.metric = new_metric.into();
    }

    pub fn update_rationale<T: Into<String>>(&mut self, new_rationale: T) {
        self.rationale = new_rationale.into();
    }
}
