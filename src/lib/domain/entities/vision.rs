use crate::domain::entities::VisionId;

#[derive(Debug, Clone)]
pub struct Vision {
    pub vision_id: VisionId,
    pub statement: String,
    pub timeframe_years: u8,
}

impl Vision {
    pub fn new<T: Into<String>>(
        statement: T, 
        timeframe_years: u8
    ) -> Self {
        Self {
            vision_id: VisionId::new(),
            statement: statement.into(),
            timeframe_years,
        }
    }

    pub fn update_statement<T: Into<String>>(&mut self, new_statement: T) {
        self.statement = new_statement.into();
    }

    pub fn update_timeframe(&mut self, new_timeframe_years: u8) {
        self.timeframe_years = new_timeframe_years;
    }
}

