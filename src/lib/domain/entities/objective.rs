use crate::domain::entities::{
    ObjectiveId,
    VisionId,
    Percent,
};

pub struct Objective {
    pub id: ObjectiveId,
    pub vision_id: VisionId,
    pub name: String,
    pub metric: String,
    pub target: Percent,
    pub current: Percent,
}

impl Objective {
    pub fn new<T: Into<String>>(
        vision_id: VisionId,
        name: T,
        metric: T,
        target: Percent,
        current: Percent,
    ) -> Self {
        Self {
            id: ObjectiveId::new(),
            vision_id,
            name: name.into(),
            metric: metric.into(),
            target,
            current,
        }
    }

    pub fn update_current(&mut self, new_current: Percent) {
        self.current = new_current;
    }

    pub fn update_target(&mut self, new_target: Percent) {
        self.target = new_target;
    }

    pub fn update_name<T: Into<String>>(&mut self, new_name: T) {
        self.name = new_name.into();
    }

    pub fn update_metric<T: Into<String>>(&mut self, new_metric: T) {
        self.metric = new_metric.into();
    }
}
