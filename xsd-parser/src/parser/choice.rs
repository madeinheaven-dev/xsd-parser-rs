use roxmltree::Node;

use crate::parser::{
    types::{Enum, EnumSource, RsEntity},
    xsd_elements::{ElementType, XsdNode},
};

pub fn parse(choice: &Node) -> RsEntity {
    let enum_cases = choice
        .children()
        .filter(|n| n.is_element() && n.xsd_type() == ElementType::Element)
        .map(|n| match n.parse(choice) {
            RsEntity::EnumCase(case) => case,
            _ => unreachable!("Elements in choice must be a enum variants"),
        })
        .collect();

    RsEntity::Enum(Enum {
        cases: enum_cases,
        type_name: "String".to_string(),
        source: EnumSource::Choice,
        ..Default::default()
    })
}
