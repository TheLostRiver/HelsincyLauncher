//! 提供跨模块复用的 ISO UTC 时间包装类型及基础转换能力。

use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 包装统一的 UTC 时间戳表示，供跨模块契约与投影复用。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IsoDateTime(DateTime<Utc>);

impl IsoDateTime {
    /// 生成当前 UTC 时间的统一包装表示。
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// 暴露底层 `chrono::DateTime<Utc>` 只读视图供适配层桥接。
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