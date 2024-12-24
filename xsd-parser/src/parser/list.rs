use roxmltree::Node;

use crate::parser::{
    constants::attribute,
    types::{RsEntity, TupleStruct, TypeModifier},
    utils::find_child,
    xsd_elements::XsdNode,
};

pub fn parse(list: &Node) -> RsEntity {
    let mut result = match list.attribute(attribute::ITEM_TYPE) {
        Some(item_type) => TupleStruct { type_name: item_type.to_string(), ..Default::default() },
        None => {
            let nested_simple_type = find_child(list, "simpleType").expect(
                "itemType not allowed if the content contains a simpleType element. Otherwise, required."
            );

            match nested_simple_type.parse(list) {
                RsEntity::Enum(en) => TupleStruct {
                    type_name: en.name.clone(),
                    subtypes: vec![RsEntity::Enum(en)],
                    ..Default::default()
                },
                RsEntity::TupleStruct(ts) => ts,
                _ => unreachable!(),
            }
        }
    };
    result.type_modifiers.push(TypeModifier::Array);
    RsEntity::TupleStruct(result)
}
