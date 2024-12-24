use std::cell::RefCell;

use roxmltree::Node;

use crate::parser::{
    types::{RsEntity, Struct, StructField, TypeModifier},
    utils::{enum_to_field, get_documentation, get_parent_name},
    xsd_elements::{ElementType, XsdNode},
};

pub fn parse(sequence: &Node, parent: &Node) -> RsEntity {
    let name = get_parent_name(sequence);
    RsEntity::Struct(Struct {
        name: name.into(),
        comment: get_documentation(parent),
        subtypes: vec![],
        fields: RefCell::new(elements_to_fields(sequence, name)),
        ..Default::default()
    })
}

fn elements_to_fields(sequence: &Node, parent_name: &str) -> Vec<StructField> {
    sequence
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .flat_map(|n| match n.parse(sequence) {
            RsEntity::StructField(mut sf) => {
                if sf.type_name.ends_with(parent_name) {
                    sf.type_modifiers.push(TypeModifier::Recursive)
                }
                vec![sf]
            }
            RsEntity::Enum(mut en) => {
                en.name = format!("{}Choice", parent_name);
                vec![enum_to_field(en)]
            }
            RsEntity::Struct(s) => s.fields.borrow().clone(),
            _ => unreachable!("\nError: {:?}\n{:?}", n, n.parse(sequence)),
        })
        .collect()
}
