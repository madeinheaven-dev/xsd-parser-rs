use inflector::Inflector;
use roxmltree::Node;

use crate::parser::{
    constants::attribute,
    types::{Import, RsEntity},
};

pub fn parse(node: &Node) -> RsEntity {
    let location = node.attribute(attribute::SCHEMA_LOCATION).unwrap_or("");
    let location =
        location.split(".").nth(0).expect(&format!("Weird name {}", location)).to_snake_case();

    RsEntity::Import(Import {
        name: node.attribute(attribute::NAMESPACE).unwrap_or("").into(),
        location: location,
        comment: None,
    })
}
