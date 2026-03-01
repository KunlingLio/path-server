use std::path::PathBuf;
use std::collections::HashSet;

use tower_lsp::lsp_types::{self, Url};
use tokio::sync::RwLock;

use crate::logger::*;

#[derive(Debug)]
pub struct PathResolver {
    workspace_root: RwLock<HashSet<lsp_types::Url>>,
}

impl PathResolver {
    pub fn new() -> Self {
        PathResolver { workspace_root: RwLock::new(HashSet::new()) }
    }

    pub async fn add_workspace_root(&self, url: &Url) {
        let mut roots = self.workspace_root.write().await;
        roots.insert(url.clone());
    }

    pub async fn remove_workspace_root(&self, url: &Url) {
        let mut roots = self.workspace_root.write().await;
        roots.remove(url);
    }

    pub async fn complete(&self, input: &str) -> Vec<PathBuf> {
        let roots = self.workspace_root.read().await;
        let mut completions = Vec::new();

        info(format!("Completing path for input: '{}'", input)).await;
        for root in roots.iter() {
            if let Ok(root_path) = root.to_file_path() {
                let candidate_path = root_path.join(input);
                if candidate_path.exists() {
                    let completion = candidate_path.strip_prefix(&root_path).unwrap_or(&candidate_path).to_path_buf();
                    completions.push(completion);
                }
            }
        }
        completions
    }
}
