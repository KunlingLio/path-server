//! Extract path and string tokens from dockerfile

use crate::document::Language;

use super::PathCandidate;

use super::super::unescape::unescape;

pub fn extract_strings(
    source: &str,
    node: &tree_sitter::Node,
    language: &Language,
) -> Vec<PathCandidate> {
    assert_eq!(language, &Language::dockerfile);

    let mut paths = Vec::new();
    // check if this node is a string
    if is_string_node(node) {
        paths.extend(extract_string_content(source, node));
    }

    // recursively process children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        paths.extend(extract_strings(source, &child, language));
    }
    paths
}

fn is_string_node(node: &tree_sitter::Node) -> bool {
    matches!(
        node.kind(),
        "path" | "json_string" | "unquoted_string" | "string" | "shell_fragment"
    )
}

fn extract_string_content(source: &str, node: &tree_sitter::Node) -> Vec<PathCandidate> {
    match node.kind() {
        "path" => {
            let candidate = PathCandidate {
                content: source[node.start_byte()..node.end_byte()].to_string(),
                start_byte: node.start_byte(),
                end_byte: node.end_byte(),
            };
            vec![candidate]
        }
        "json_string" => {
            let node_text = &source[node.start_byte()..node.end_byte()];
            let node_chars = node_text.chars().collect::<Vec<char>>();
            let mut cursor = node.walk();
            let children: Vec<tree_sitter::Node> = node.children(&mut cursor).collect();
            let children_text: Vec<String> = children
                .iter()
                .map(|child| source[child.start_byte()..child.end_byte()].to_string())
                .collect();
            // traverse char in node_text and the children to match them
            let mut candidate = String::new();
            let mut children_index = 0;
            let mut text_index = 0;
            while text_index < node_text.len() {
                let cur_char = node_chars[text_index];
                let cur_children_text = &children_text[children_index];
                if cur_char == cur_children_text.chars().next().unwrap_or('\0') {
                    let cur_children = children[children_index];
                    match cur_children.kind() {
                        "\"" | "'" => { // skip the quote
                        }
                        "escape_sequence" => {
                            // unescape the escape sequence
                            let unescaped = unescape(cur_children_text).unwrap();
                            candidate.push_str(&unescaped);
                        }
                        _ => {
                            candidate.push_str(cur_children_text);
                        }
                    }
                    children_index += 1;
                } else {
                    candidate.push(cur_char);
                }
                text_index += 1;
            }
            vec![PathCandidate {
                content: candidate,
                start_byte: node.start_byte(),
                end_byte: node.end_byte(),
            }]
        }
        "unquoted_string" | "string" => {
            let candidate = PathCandidate {
                content: source[node.start_byte()..node.end_byte()].to_string(),
                start_byte: node.start_byte(),
                end_byte: node.end_byte(),
            };
            vec![candidate]
        }
        "shell_fragment" => {
            // need to split by spaces
            let node_text = &source[node.start_byte()..node.end_byte()];
            node_text
                .split_whitespace()
                .map(|part| PathCandidate {
                    content: part.to_string(),
                    start_byte: node.start_byte() + node_text.find(part).unwrap_or(0),
                    end_byte: node.start_byte() + node_text.find(part).unwrap_or(0) + part.len(),
                })
                .collect()
        }
        _ => {
            unreachable!("Unexpected node kind: {}", node.kind());
        }
    }
}
