//! 提供跨模块共享的时间获取抽象与默认系统时钟实现。

use crate::time::IsoDateTime;

/// 抽象当前时间来源，供需要可替换时间依赖的模块复用。
pub trait Clock: Send + Sync {
    /// 返回当前 UTC 时间戳的统一基础表示。
    fn now(&self) -> IsoDateTime;
}

#[derive(Debug, Default, Clone, Copy)]
/// 使用系统当前 UTC 时间作为实现的默认时钟。
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> IsoDateTime {
        IsoDateTime::now()
    }
}