//! Async concurrent file system access api wrapper

use std::path::{Path, PathBuf};
use tokio::fs;
use tower_lsp_server::ls_types;

use crate::error::*;

/// Check if a path exists
pub async fn exists(path: impl AsRef<Path>) -> bool {
    fs::try_exists(path).await.unwrap_or(false)
}

/// Check if dir
pub async fn is_dir(path: impl AsRef<Path>) -> bool {
    if let Ok(metadata) = fs::metadata(path).await {
        metadata.is_dir()
    } else {
        false
    }
}

/// Return entries for a dir
pub async fn read_dir(path: impl AsRef<Path>) -> PathServerResult<Vec<fs::DirEntry>> {
    let mut entries = fs::read_dir(path).await?;
    let mut files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        files.push(entry);
    }

    Ok(files)
}

pub fn url_to_path(url: &ls_types::Uri) -> PathServerResult<Option<PathBuf>> {
    match url.scheme().as_str() {
        "file" => url
            .to_file_path()
            .map(|path| path.into_owned())
            .ok_or(PathServerError::InvalidPath(format!(
                "Failed to convert URL to file path: {}",
                url.as_str()
            )))
            .map(Some),
        "untitled" => Ok(None),
        _ => Err(PathServerError::Unsupported(format!(
            "Non-local url is not supported: {}",
            url.as_str()
        ))),
    }
}

pub fn is_hidden_file(path: &Path) -> PathServerResult<bool> {
    let Some(is_unix_hidden) = path
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
    else {
        return Err(PathServerError::InvalidPath(format!(
            "{} do not contained file name, cannot check hidden or not",
            path.display()
        )));
    };
    if is_unix_hidden {
        return Ok(true);
    }
    #[cfg(windows)]
    {
        if hf::is_hidden(path)? {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn path_to_url(path: &PathBuf) -> PathServerResult<ls_types::Uri> {
    ls_types::Uri::from_file_path(path).ok_or(PathServerError::InvalidPath(format!(
        "Failed to convert file path to URL: {}",
        path.display()
    )))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_url_to_path() {
        // valid file url
        #[cfg(not(windows))]
        let url_str = "file:///tmp";
        #[cfg(windows)]
        let url_str = "file:///C:/tmp";
        let url = ls_types::Uri::from_str(url_str).unwrap();
        let path = url_to_path(&url).unwrap().unwrap();
        assert!(path.ends_with("tmp"));

        // non-file scheme should error
        let url = ls_types::Uri::from_str("http://example.com").unwrap();
        let err = url_to_path(&url).unwrap_err();
        match err {
            PathServerError::Unsupported(_) => {}
            _ => assert!(false, "expected Unsupported error, got: {}", err),
        }
    }
}
