//! 定义跨 crate 复用的统一应用结果别名。

use crate::error::AppError;

/// 基于统一 `AppError` 契约的共享结果类型别名。
pub type AppResult<T> = Result<T, AppError>;