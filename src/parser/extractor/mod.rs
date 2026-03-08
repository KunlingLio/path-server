use crate::{document::Document, parser::PathCandidate};
mod general;
mod tree_sitter;

pub use tree_sitter::update_tree;

pub fn extract_string(document: &Document) -> Vec<PathCandidate> {
    // crate::logger::debug_sync("@@@".into());
    let res = tree_sitter::extract_strings(document);
    if res.is_none() {
        // fall back to general parser
        general::extract_string(document).unwrap_or_default()
    } else {
        res.unwrap()
    }
}
