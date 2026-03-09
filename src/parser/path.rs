//! Parsers for document path parsing.
use std::vec::Vec;

use crate::document::Document;

use super::PathCandidate;
use super::extractor::extract_string;

pub fn parse_document(document: &Document) -> Vec<Vec<PathCandidate>> {
    extract_string(document)
        .into_iter()
        .map(extract_paths_from_string)
        .collect()
}

/// Try to extract paths from a string token,
/// return candidates, from high priority to low priority
fn extract_paths_from_string(path_ref: PathCandidate) -> Vec<PathCandidate> {
    let mut results = Vec::new();
    let content = &path_ref.content;

    // 1. whole string is a path or not
    if content.contains('/') || content.contains('\\') {
        results.push(path_ref.clone());
    }

    // 2. the part of string (split by space) is a path or not
    if let Some(pos) = content.rfind(' ') {
        let sub_content = &content[pos + 1..];
        if sub_content.contains('/') || sub_content.contains('\\') {
            results.push(path_ref.slice(pos + 1, path_ref.len()));
        }
    }

    results
}
