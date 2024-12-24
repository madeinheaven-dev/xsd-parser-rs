use roxmltree::Node;

use crate::parser::{
    types::RsEntity,
    xsd_elements::{ElementType, XsdNode},
};

pub fn parse(node: &Node) -> RsEntity {
    let content = node
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .last()
        .expect("Content in complexContent required");

    content.parse(node)
}
