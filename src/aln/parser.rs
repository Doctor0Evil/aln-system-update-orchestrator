use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlnUpdatePlan {
    pub version: String,
    pub policy_file: String,
    pub features: Vec<String>,
}

#[derive(Debug, Error)]
pub enum LoadAlnError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}

impl AlnUpdatePlan {
    pub fn from_file(path: &str) -> Result<Self, LoadAlnError> {
        let raw = fs::read_to_string(path)?;
        // Minimal, placeholder ALN parsing: in a full implementation,
        // parse the ALN AST; here we hard-wire from known file.
        if raw.contains("@ALN_UPDATE_SYSTEM") {
            Ok(Self {
                version: "1.0.1.7".to_string(),
                policy_file: "aln/system_update_policy_v1.7.rego".to_string(),
                features: vec![
                    "repo_tracking".into(),
                    "commit_automation".into(),
                    "manifest_scaling".into(),
                ],
            })
        } else {
            Err(LoadAlnError::Parse("ALN_UPDATE_SYSTEM block not found".into()))
        }
    }
}
