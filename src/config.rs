use std::convert::TryFrom;
use std::path::PathBuf;

use serde::Deserialize;
use tower_lsp::lsp_types;

use crate::logger::*;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub completion: Completion,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Completion {
    /// Max results shown in completion, 0 indicate no limit
    #[serde(alias = "maxResults")]
    pub max_results: usize,

    /// Whether to show hidden files in completion
    #[serde(alias = "showHiddenFiles")]
    pub show_hidden_files: bool,

    /// List of paths to exclude from completion
    /// Supports glob patterns
    pub exclude: Vec<String>,

    /// Base paths for relative path completion
    /// Supports `${workspaceFolder}`, `${document}`, `${userHome}` as placeholders
    #[serde(alias = "basePath")]
    pub base_path: Vec<String>,
}

impl Completion {
    pub fn iter_base_path(
        &self,
        workspace_folders: &[String],
        document_parent: &Option<String>,
        user_home: &Option<String>,
    ) -> Vec<PathBuf> {
        let mut expanded_paths = vec![];
        for path in &self.base_path {
            if path.contains("${workspaceFolder}") {
                for workspace_folder in workspace_folders {
                    let expanded = path.replace("${workspaceFolder}", workspace_folder);
                    expanded_paths.push(PathBuf::from(expanded));
                }
            } else if path.contains("${document}") {
                if document_parent.is_some() {
                    let expanded = path.replace("${document}", document_parent.as_deref().unwrap());
                    expanded_paths.push(PathBuf::from(expanded));
                }
            } else if path.contains("${userHome}") {
                if user_home.is_some() {
                    let expanded = path.replace("${userHome}", user_home.as_deref().unwrap());
                    expanded_paths.push(PathBuf::from(expanded));
                }
            } else {
                expanded_paths.push(PathBuf::from(path));
            }
        }
        expanded_paths
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            completion: Completion {
                max_results: 0,
                show_hidden_files: true,
                exclude: vec!["**/node_modules/**".into(), "**/.git/**".into()],
                base_path: vec!["${workspaceFolder}".into(), "${document}".into()],
            },
        }
    }
}

impl TryFrom<serde_json::Value> for Config {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}

pub async fn get(client: &tower_lsp::Client) -> Config {
    let configs = client
        .configuration(vec![lsp_types::ConfigurationItem {
            scope_uri: None,
            section: Some("path-server".to_string()),
        }])
        .await;
    let Ok(configs) = configs else {
        info(format!(
            "Failed to get configuration:{}, use default",
            configs.unwrap_err()
        ))
        .await;
        return Default::default();
    };
    assert!(configs.len() == 1);
    let Ok(config) = Config::try_from(configs[0].clone()) else {
        info(format!(
            "Failed to parse configuration:{}, use default",
            configs[0].clone()
        ))
        .await;
        return Default::default();
    };
    config
}
