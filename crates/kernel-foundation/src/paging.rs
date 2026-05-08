//! 定义跨模块复用的分页游标、请求与结果契约。

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
/// 表示后续分页请求可继续使用的稳定游标值。
pub struct PageCursor(String);

impl PageCursor {
    /// 使用原始游标文本构造分页游标值对象。
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// 读取当前游标携带的原始文本值。
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for PageCursor {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// 表示一次分页查询的通用输入契约。
pub struct PageRequest {
    /// 请求当前页时希望返回的最大条目数。
    pub limit: u32,
    /// 可选的起始游标；为空时表示从第一页开始读取。
    pub cursor: Option<PageCursor>,
}

impl PageRequest {
    /// 组合分页上限和可选游标，构造统一分页请求。
    pub fn new(limit: u32, cursor: Option<PageCursor>) -> Self {
        Self { limit, cursor }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// 表示一页结果项以及继续翻页所需的后续游标。
pub struct PageSlice<T> {
    /// 当前页实际返回的结果项集合。
    pub items: Vec<T>,
    /// 下一页游标；为空时表示没有更多结果。
    pub next_cursor: Option<PageCursor>,
}

impl<T> PageSlice<T> {
    /// 用结果项和后续游标构造一页统一分页结果。
    pub fn new(items: Vec<T>, next_cursor: Option<PageCursor>) -> Self {
        Self { items, next_cursor }
    }
}