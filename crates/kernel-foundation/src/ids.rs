//! 提供跨模块复用的字符串标识类型及统一生成/访问辅助。

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// 为一组同形的字符串 ID 生成统一包装和基础辅助 API。
macro_rules! define_string_id {
    ($(#[$type_meta:meta])* $name:ident) => {
        $(#[$type_meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// 使用现有字符串值构造标识包装。
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            /// 生成新的 UUID 字符串标识。
            pub fn generate() -> Self {
                Self(Uuid::new_v4().to_string())
            }

            /// 返回底层字符串值的只读视图。
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

define_string_id!(
    /// 表示账户实体的稳定字符串标识。
    AccountId
);
define_string_id!(
    /// 表示 Fab 资源或资产实体的稳定字符串标识。
    AssetId
);
define_string_id!(
    /// 表示跨模块关联链路的稳定字符串标识。
    CorrelationId
);
define_string_id!(
    /// 表示后台作业实体的稳定字符串标识。
    JobId
);