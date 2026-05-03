use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IsoDateTime(DateTime<Utc>);

impl IsoDateTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn as_datetime(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl Display for IsoDateTime {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0.to_rfc3339())
    }
}

impl From<DateTime<Utc>> for IsoDateTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<IsoDateTime> for DateTime<Utc> {
    fn from(value: IsoDateTime) -> Self {
        value.0
    }
}