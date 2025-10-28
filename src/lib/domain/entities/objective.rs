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
