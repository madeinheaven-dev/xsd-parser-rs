mod all;
mod any;
mod any_attribute;
mod attribute;
mod attribute_group;
mod choice;
mod complex_content;
mod complex_type;
pub mod constants;
mod element;
mod extension;
mod group;
mod import;
mod list;
mod restriction;
pub mod schema;
mod sequence;
mod simple_content;
mod simple_type;
mod tests;
pub mod types;
mod union;
mod utils;
pub mod xsd_elements;

use std::collections::HashMap;

use inflector::Inflector;
use types::Group;

use crate::parser::types::{RsEntity, RsFile};

fn unveil_references<'input>(
    file: &RsFile<'input>,
    groups: &HashMap<String, types::Group>,
) -> RsFile<'input> {
    let c = file.clone();

    file.types.iter().for_each(|t| match t {
        types::RsEntity::Struct(s) => s.fields.borrow().iter().for_each(|f| {
            let group_reference = f.group_reference.clone().unwrap_or_default();
            if let false = group_reference.is_empty() {
                let key = group_reference.split(":").last().unwrap_or_default();
                let reference = groups.get(key).expect(&format!("Cant find group {}", group_reference));
                let typo = reference.typo.clone();
                if let types::RsEntity::Struct(s2) = *typo {
                    if let types::RsEntity::Struct(o) = c.types.iter().find(|t| t.name() == s.name).expect(&format!("Cant find type {}", s.name)) {
                        o.fields.borrow_mut().append(&mut s2.fields.borrow_mut());
                    }
                }
            }
        }),
        _ => (),
    });
    c
}

// FIXME: Actually pass up errors
#[allow(clippy::result_unit_err)]
pub fn parse(text: &str) -> Result<RsFile, ()> {
    let doc = roxmltree::Document::parse(text).expect("Parse document error");
    let root = doc.root();

    let mut map = HashMap::new();

    let schema =
        root.children().filter(|e| e.is_element()).last().expect("Schema element is required");

    let schema_rs: RsFile = crate::parser::schema::parse(&schema);
    for ty in &schema_rs.types {
        if let RsEntity::Struct(st) = ty {
            map.extend(st.get_types_map());
        }
    }
    for ag in &schema_rs.attribute_groups {
        if let RsEntity::Struct(st) = ag {
            map.extend(st.get_types_map());
        }
    }
    for ty in &schema_rs.types {
        if let RsEntity::Struct(st) = ty {
            st.extend_base(&map);
            st.extend_attribute_group(&map);
        }
    }

    Ok(schema_rs)
}

pub fn parse_files<'input>(
    files: &'input HashMap<String, String>,
) -> Result<HashMap<String, RsFile<'input>>, ()> {
    let mut rs_files: HashMap<String, RsFile> = HashMap::new();

    for (k, v) in files {
        match parse(v) {
            Ok(f) => {
                rs_files.insert(k.to_string(), f);
            }
            Err(err) => panic!("{} => {:?}", k, err),
        }
    }

    let groups: HashMap<String, Group> =
        rs_files.iter().flat_map(|rsf| rsf.1.groups.clone()).collect();

    let mut res: HashMap<String, RsFile> = HashMap::new();

    for (k, rsf) in rs_files {
        res.insert(
            std::path::Path::new(&k).with_extension("").to_str().unwrap().to_snake_case(),
            unveil_references(&rsf, &groups),
        );
    }

    Ok(res)
}
