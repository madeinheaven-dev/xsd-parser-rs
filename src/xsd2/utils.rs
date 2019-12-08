use roxmltree::*;

pub fn find_child<'a, 'input>(node: &Node<'a, 'input>, tag_name: &str) -> Option<Node<'a, 'input>> {
    node.children().find(|e| e.is_element() && e.tag_name().name() == tag_name)
}

pub fn find_element<'a, 'input>(node: &Node<'a, 'input>, tag_name: &str) -> Option<Node<'a, 'input>> {
    match node.
        traverse().
        find(|e| match e {
            Edge::Open(x) => x.is_element() && x.tag_name().name() == tag_name,
            _ => false
        })
    {
        Some(Edge::Open(node)) => Some(node.clone()),
        _ => None
    }
}

pub fn get_documentation<'a>(node: &Node<'a, '_>) -> Option<&'a str> {
    match find_element(node, "documentation") {
        Some(node) => node.text(),
        None => None
    }
}

pub fn get_node_type<'a>(node: &Node<'a, '_>) -> &'a str {
    match node.attribute("type") {
        Some(name) => name,
        None => match node.attribute("ref") {
            Some(s) => s,
            None => "_UNSUPPORTED_TYPE"
        }
    }
}

pub fn get_node_name<'a>(node: &Node<'a, '_>) -> &'a str {
    match node.attribute("name") {
        Some(name) => name,
        None => match node.attribute("ref") {
            Some(s) => s,
            None => "_UNSUPPORTED_NAME"
        }
    }
}

pub type MinOccurs = usize;
pub enum MaxOccurs {
    Bounded(usize),
    Unbounded
}