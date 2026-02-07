use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlnUpdatePlan {
    pub version: String,
    pub components: AlnComponentConfig,
    pub interop: AlnInteropConfig,
    pub render: AlnRenderConfig,
    pub rego_exec: AlnRegoExecConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlnComponentConfig {
    pub game_engine: String,
    pub ai_chat_ui: String,
    pub renderers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlnInteropConfig {
    pub cross_link: String,
    pub maintain_func: bool,
    pub enable_lan: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlnRenderConfig {
    pub mode: String,
    pub merge_sources: bool,
    pub playable_platforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlnRegoExecConfig {
    pub always_active: bool,
    pub policy: String,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlnAction {
    pub name: String,
    pub params: serde_json::Value,
}
