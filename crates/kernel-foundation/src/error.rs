use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ids::CorrelationId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppErrorSeverity {
    Info,
    Warning,
    Error,
    Fatal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{code}: {message}")]
pub struct AppError {
    pub code: String,
    pub message: String,
    pub retryable: bool,
    pub severity: AppErrorSeverity,
    pub correlation_id: CorrelationId,
}

impl AppError {
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