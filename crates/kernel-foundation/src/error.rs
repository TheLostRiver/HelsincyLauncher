//! 定义后端统一错误契约及其稳定投影字段。

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ids::CorrelationId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// 标记错误对当前流程和用户的影响级别。
pub enum AppErrorSeverity {
    /// 非失败性提示或低风险异常状态。
    Info,
    /// 功能受限但当前流程仍可继续。
    Warning,
    /// 当前操作失败，需要用户关注或重试。
    Error,
    /// 核心流程被阻断，当前能力不可继续。
    Fatal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{code}: {message}")]
/// 供后端各层收敛并向传输层投影的统一应用错误对象。
pub struct AppError {
    /// 稳定的错误分类键，前端应依赖它而不是 message 文本做判断。
    pub code: String,
    /// 适合投影到 UI 的简短错误说明。
    pub message: String,
    /// 指示当前失败是否存在合理重试空间。
    pub retryable: bool,
    /// 描述这次失败或异常状态的影响级别。
    pub severity: AppErrorSeverity,
    /// 用于串联日志、命令和 diagnostics 的关联标识。
    pub correlation_id: CorrelationId,
}

impl AppError {
    /// 使用统一错误契约字段构造一个可跨层传递的应用错误。
    pub fn new(
        code: impl Into<String>,
        message: impl Into<String>,
        retryable: bool,
        severity: AppErrorSeverity,
        correlation_id: CorrelationId,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            retryable,
            severity,
            correlation_id,
        }
    }
}