use std::{fmt, str::FromStr};
use uuid::Uuid;

macro_rules! define_id {
    ($name:ident, $prefix:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn new() -> Self {
                let uuid = Uuid::new_v4();
                Self(format!("{}:{}", $prefix, uuid))
            }

            /// It should have the prefix and check the second is uuid
            pub fn from_string<S: Into<String>>(s: S) -> Self {
                let s = s.into();
                assert!(
                    s.starts_with($prefix),
                    "ID must start with the prefix {}",
                    $prefix
                );
                assert!(
                    s.split(':')
                        .nth(1)
                        .and_then(|part| Uuid::parse_str(part).ok())
                        .is_some(),
                    "ID must contain a valid UUID after the prefix"
                );
                Self(s)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn uuid_part(&self) -> Option<Uuid> {
                self.0.split(':')
                    .nth(1)
                    .and_then(|part| Uuid::parse_str(part).ok())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl FromStr for $name {
            type Err = std::convert::Infallible;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self::from_string(s.to_owned()))
            }
        }
    };
}

define_id!(VisionId, "VI");
define_id!(ObjectiveId, "OB");
define_id!(GoalId, "GL");
define_id!(InitiativeId, "IN");
define_id!(TaskId, "TS");
