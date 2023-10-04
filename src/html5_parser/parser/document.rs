use crate::html5_parser::node::{Node, NodeData, NodeId};
use crate::html5_parser::node_arena::NodeArena;
use crate::html5_parser::parser::quirks::QuirksMode;
use std::fmt;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum DocumentType {
    HTML,
    IframeSrcDoc,
}

pub struct Document {
    arena: NodeArena,
    pub doctype: DocumentType,   // Document type
    pub quirks_mode: QuirksMode, // Quirks mode
}

impl Document {
    pub(crate) fn print_nodes(&self) {
        self.arena.print_nodes();
    }
}

impl Default for Document {
    fn default() -> Self {
        Self {
            arena: NodeArena::new(),
            doctype: DocumentType::HTML,
            quirks_mode: QuirksMode::NoQuirks,
        }
    }
}

impl Document {
    // Creates a new document
    pub fn new() -> Self {
        let mut arena = NodeArena::new();
        arena.add_node(Node::new_document());
        Self {
            arena,
            doctype: DocumentType::HTML,
            quirks_mode: QuirksMode::NoQuirks,
        }
    }

    // Fetches a node by id or returns None when no node with this ID is found
    pub fn get_node_by_id(&self, node_id: NodeId) -> Option<&Node> {
        self.arena.get_node(node_id)
    }

    pub fn get_mut_node_by_id(&mut self, node_id: NodeId) -> Option<&mut Node> {
        self.arena.get_mut_node(node_id)
    }

    // Add to the document
    pub fn add_node(&mut self, node: Node, parent_id: NodeId) -> NodeId {
        let node_id = self.arena.add_node(node);
        self.arena.attach_node(parent_id, node_id);
        node_id
    }

    pub fn append(&mut self, node_id: NodeId, parent_id: NodeId) {
        self.arena.attach_node(parent_id, node_id);
    }

    pub fn relocate(&mut self, node_id: NodeId, parent_id: NodeId) {
        // Remove the node from its current parent (if any)
        let cur_parent_id = self.arena.get_node(node_id).expect("node not found").parent;
        if let Some(parent_node_id) = cur_parent_id {
            let cur_parent = self
                .arena
                .get_mut_node(parent_node_id)
                .expect("node not found");
            cur_parent.children.retain(|&x| x != node_id);
        }

        // Add the node to the new parent as a child, and update the node's parent
        self.arena
            .get_mut_node(parent_id)
            .unwrap()
            .children
            .push(node_id);
        self.arena.get_mut_node(node_id).unwrap().parent = Some(parent_id);
    }

    // return the root node
    pub fn get_root(&self) -> &Node {
        self.arena
            .get_node(NodeId::root())
            .expect("Root node not found !?")
    }
}

impl Document {
    /// Print a node and all its children in a tree-like structure
    pub fn print_tree(&self, node: &Node, prefix: String, last: bool, f: &mut fmt::Formatter<'_>) {
        let mut buffer = prefix.clone();
        if last {
            buffer.push_str("└─ ");
        } else {
            buffer.push_str("├─ ");
        }

        match &node.data {
            NodeData::Document => {
                _ = writeln!(f, "{}Document", buffer);
            }
            NodeData::Text { value } => {
                _ = writeln!(f, "{}\"{}\"", buffer, value);
            }
            NodeData::Comment { value } => {
                _ = writeln!(f, "{}<!-- {} -->", buffer, value);
            }
            NodeData::Element { name, attributes } => {
                _ = write!(f, "{}<{}", buffer, name);
                for (key, value) in attributes.iter() {
                    _ = write!(f, " {}={}", key, value);
                }
                _ = writeln!(f, ">");
            }
        }

        if prefix.len() > 40 {
            _ = writeln!(f, "...");
            return;
        }

        let mut buffer = prefix;
        if last {
            buffer.push_str("   ");
        } else {
            buffer.push_str("│  ");
        }

        let len = node.children.len();
        for (i, child) in node.children.iter().enumerate() {
            let child = self.arena.get_node(*child).expect("Child not found");
            self.print_tree(child, buffer.clone(), i == len - 1, f);
        }
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print_tree(self.get_root(), "".to_string(), true, f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::html5_parser::node::HTML_NAMESPACE;
    use std::collections::HashMap;

    #[ignore]
    #[test]
    fn test_document() {
        let mut document = super::Document::new();
        let root_id = document.get_root().id;
        let html_id = document.add_node(
            super::Node::new_element("html", HashMap::new(), HTML_NAMESPACE),
            root_id,
        );
        let head_id = document.add_node(
            super::Node::new_element("head", HashMap::new(), HTML_NAMESPACE),
            html_id,
        );
        let body_id = document.add_node(
            super::Node::new_element("body", HashMap::new(), HTML_NAMESPACE),
            html_id,
        );
        let title_id = document.add_node(
            super::Node::new_element("title", HashMap::new(), HTML_NAMESPACE),
            head_id,
        );
        let title_text_id = document.add_node(super::Node::new_text("Hello world"), title_id);
        let p_id = document.add_node(
            super::Node::new_element("p", HashMap::new(), HTML_NAMESPACE),
            body_id,
        );
        let p_text_id = document.add_node(super::Node::new_text("This is a paragraph"), p_id);
        let p_comment_id = document.add_node(super::Node::new_comment("This is a comment"), p_id);
        let p_text2_id =
            document.add_node(super::Node::new_text("This is another paragraph"), p_id);
        let p_text3_id =
            document.add_node(super::Node::new_text("This is a third paragraph"), p_id);
        let p_text4_id =
            document.add_node(super::Node::new_text("This is a fourth paragraph"), p_id);
        let p_text5_id =
            document.add_node(super::Node::new_text("This is a fifth paragraph"), p_id);
        let p_text6_id =
            document.add_node(super::Node::new_text("This is a sixth paragraph"), p_id);
        let p_text7_id =
            document.add_node(super::Node::new_text("This is a seventh paragraph"), p_id);
        let p_text8_id =
            document.add_node(super::Node::new_text("This is a eighth paragraph"), p_id);
        let p_text9_id =
            document.add_node(super::Node::new_text("This is a ninth paragraph"), p_id);

        document.append(p_text9_id, p_id);
        document.append(p_text8_id, p_id);
        document.append(p_text7_id, p_id);
        document.append(p_text6_id, p_id);
        document.append(p_text5_id, p_id);
        document.append(p_text4_id, p_id);
        document.append(p_text3_id, p_id);
        document.append(p_text2_id, p_id);
        document.append(p_comment_id, p_id);
        document.append(p_text_id, p_id);
        document.append(p_id, body_id);
        document.append(title_text_id, title_id);
        document.append(title_id, head_id);
        document.append(head_id, html_id);
        document.append(body_id, html_id);
        document.append(html_id, root_id);

        assert_eq!(
            format!("{}", document),
            r#"Document
  └─ <html>
    └─ <head>
      └─ <title>
        └─ Hello world
    └─ <body>
      └─ <p>
      └─ This is a paragraph
      └─ <!-- This is a comment -->
      └─ This is another paragraph
      └─ This is a third paragraph
      └─ This is a fourth paragraph
      └─ This is a fifth paragraph
      └─ This is a sixth paragraph
      └─ This is a seventh paragraph
      └─ This is a eighth paragraph
      └─ This is a ninth paragraph
      "#
        );
    }
}