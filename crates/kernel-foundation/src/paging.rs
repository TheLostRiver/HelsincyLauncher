use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PageCursor(String);

impl PageCursor {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

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
pub struct PageRequest {
    pub limit: u32,
    pub cursor: Option<PageCursor>,
}

impl PageRequest {
    pub fn new(limit: u32, cursor: Option<PageCursor>) -> Self {
        Self { limit, cursor }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageSlice<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<PageCursor>,
}

impl<T> PageSlice<T> {
    pub fn new(items: Vec<T>, next_cursor: Option<PageCursor>) -> Self {
        Self { items, next_cursor }
    }
}