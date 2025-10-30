use anyhow::Result;
use crate::domain::entities::{Vision, VisionId};

pub struct CreateVision { 
    pub description: String
}

pub trait VisionRepository: Send + Sync {
    fn get(&self) -> Result<Vision>;
    fn get_by_id<T: Into<VisionId>>(&self, id: T) -> Result<Vision>;
    fn create(&mut self, vision: CreateVision) -> Result<Vision>;
    fn update(&mut self, vision: Vision) -> Result<()>;
    fn delete<T: Into<VisionId>>(&mut self, id: T) -> Result<()>;
} 
