#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpicFabCatalogProviderConfig {
    base_url: String,
    client_name: String,
}

impl EpicFabCatalogProviderConfig {
    pub fn new(base_url: impl Into<String>, client_name: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client_name: client_name.into(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn client_name(&self) -> &str {
        &self.client_name
    }
}

#[derive(Debug, Clone)]
pub struct EpicFabCatalogProviderAdapter {
    config: EpicFabCatalogProviderConfig,
}

impl EpicFabCatalogProviderAdapter {
    pub fn new(config: EpicFabCatalogProviderConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &EpicFabCatalogProviderConfig {
        &self.config
    }
}