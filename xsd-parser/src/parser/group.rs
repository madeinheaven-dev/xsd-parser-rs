use roxmltree::Node;

use crate::parser::types::RsEntity;

use crate::parser::{
    types::{Group, StructField},
    utils::get_documentation,
    xsd_elements::{ElementType, XsdNode},
};

const SUPPORTED_CONTENT_TYPES: [ElementType; 3] =
    [ElementType::All, ElementType::Choice, ElementType::Sequence];

pub fn parse(node: &Node, parent: &Node) -> RsEntity {
    match parent.xsd_type() {
        ElementType::Schema => parse_global_element(node),
        _ => RsEntity::StructField(StructField {
            name: node.attr_ref().unwrap().to_string(),
            group_reference: Some(node.attr_ref().unwrap().to_string()),
            ..Default::default()
        }),
    }
}

fn parse_global_element(node: &Node) -> RsEntity {
    node.attr_type().unwrap_or("UNSUPPORTED");

    let name =
        node.attr_name().expect("Name required if the Group element is a child of the schema");

    let content_node = node
        .children()
        .filter(|n| SUPPORTED_CONTENT_TYPES.contains(&n.xsd_type()))
        .last()
        .unwrap_or_else(|| panic!("Must have content if no 'type' or 'ref' attribute: {:?}", node));

    RsEntity::Group(Group {
        name: name.to_string(),
        comment: get_documentation(&node),
        typo: Box::new(content_node.parse(&node)),
    })
}
