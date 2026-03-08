use std::path::PathBuf;

use futures::future;
use tower_lsp::lsp_types;

use crate::async_fs;
use crate::common::*;
use crate::document::Document;
use crate::logger::debug;
use crate::parser::{PathCandidate, parse_document};

pub async fn provide_document_links(
    doc: &Document,
) -> PathServerResult<Vec<lsp_types::DocumentLink>> {
    crate::logger::debug("@@@".into()).await;
    let tokens: Vec<(PathCandidate, PathBuf)> = future::join_all(
        parse_document(doc)
            .into_iter()
            .map(|candidates| async move {
                for candidate in candidates {
                    debug(format!("Checking candidate: {}", candidate.content)).await;
                    let path = PathBuf::from(&candidate.content);
                    if async_fs::exists(&path).await {
                        return Some((candidate, path));
                    }
                }
                None
            }),
    )
    .await
    .into_iter()
    .filter(Option::is_some)
    .map(|x| x.unwrap())
    .collect();

    debug(format!("Tokens: {}", tokens.len())).await;

    let mut links = vec![];
    for token in tokens {
        let candidate = token.0;
        let path = token.1;
        let start = doc.utf_16_pos(candidate.start_byte)?;
        let end = doc.utf_16_pos(candidate.end_byte)?;
        let range = lsp_types::Range::new(
            lsp_types::Position::new(start.0 as u32, start.1 as u32),
            lsp_types::Position::new(end.0 as u32, end.1 as u32),
        );

        links.push(lsp_types::DocumentLink {
            range,
            target: Some(lsp_types::Url::from_file_path(path.clone()).map_err(|_| {
                PathServerError::Unknown(format!(
                    "Failed to convert path {} into url",
                    path.display()
                ))
            })?),
            tooltip: Some("Follow path".into()),
            data: None,
        });
    }

    Ok(links)
}
