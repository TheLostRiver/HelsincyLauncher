use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 列出当前可见引擎集合时使用的查询请求。
pub struct ListEnginesRequestDto {}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 读取单个引擎当前状态时使用的查询请求。
pub struct GetEngineStatusRequestDto {
    pub engine_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 提交引擎校验作业时使用的命令请求。
pub struct RunEngineVerificationRequestDto {
    pub engine_id: String,
}
