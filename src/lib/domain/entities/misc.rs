#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Year(pub u16);

impl Year {
    pub fn new(year: u16) -> Self {
        Self(year)
    }
    
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Month(pub u8);


impl Month {
    pub fn new(month: u8) -> Self {
        assert!(month >= 1 && month <= 12, "Month must be between 1 and 12");
        Self(month)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}", self.0)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Day(pub u8);

impl Day {
    pub fn new(day: u8) -> Self {
        assert!(day >= 1 && day <= 31, "Day must be between 1 and 31");
        Self(day)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}", self.0)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Quarter { Q1, Q2, Q3, Q4}

impl Quarter {
    pub fn to_string(&self) -> String {
        match self {
            Quarter::Q1 => "Q1".to_string(),
            Quarter::Q2 => "Q2".to_string(),
            Quarter::Q3 => "Q3".to_string(),
            Quarter::Q4 => "Q4".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percent(pub f64);

impl Percent {
    pub fn new(value: f64) -> Self {
        assert!(value >= 0.0 && value <= 100.0, "Percent must be between 0 and 100");
        Self(value)
    }

    pub fn to_string(&self) -> String {
        format!("{:.2}%", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quantity(pub f64);

impl Quantity {
    pub fn new(value: f64) -> Self {
        assert!(value >= 0.0, "Quantity must be non-negative");
        Self(value)
    }

    pub fn to_string(&self) -> String {
        format!("{:.2}", self.0)
    }
}


