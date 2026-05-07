//! Fab provider adapter crate 的最小公开入口。
//!
//! 这个 shell-first 边界当前只暴露 provider 配置与适配器外壳构造能力，供
//! composition root 先装配稳定的 Fab 上游依赖；真实远程鉴权、HTTP 调用和
//! 载荷归一化仍保留给后续切片实现。

/// 组装 Epic/Fab catalog provider 适配器所需的最小配置快照。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpicFabCatalogProviderConfig {
    base_url: String,
    client_name: String,
}

impl EpicFabCatalogProviderConfig {
    /// 用基础地址和客户端标识创建 provider 配置。
    pub fn new(base_url: impl Into<String>, client_name: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client_name: client_name.into(),
        }
    }

    /// 返回当前 provider adapter 绑定的基础 API 地址。
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// 返回当前 provider 请求使用的客户端标识。
    pub fn client_name(&self) -> &str {
        &self.client_name
    }
}

/// 面向 Epic/Fab catalog 的最小 provider 适配器外壳。
#[derive(Debug, Clone)]
pub struct EpicFabCatalogProviderAdapter {
    config: EpicFabCatalogProviderConfig,
}

impl EpicFabCatalogProviderAdapter {
    /// 用既有配置创建 provider 适配器外壳。
    pub fn new(config: EpicFabCatalogProviderConfig) -> Self {
        Self { config }
    }

    /// 暴露只读配置快照，供装配和诊断路径检查当前 provider 绑定结果。
    pub fn config(&self) -> &EpicFabCatalogProviderConfig {
        &self.config
    }
}