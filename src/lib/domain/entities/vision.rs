use crate::domain::entities::VisionId;

#[derive(Debug, Clone)]
pub struct Vision {
    pub vision_id: VisionId,
    pub statement: String,
    pub timeframe_years: u8,
}

