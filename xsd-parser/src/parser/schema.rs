use roxmltree::Node;

use crate::parser::{
    types::RsFile,
    utils::target_namespace,
    xsd_elements::{ElementType, XsdNode},
};

use crate::parser::types;

pub fn parse<'input>(schema: &Node<'_, 'input>) -> RsFile<'input> {
    let mut xsd_namespaces = schema
        .namespaces()
        .filter(|namespace| namespace.uri() == "http://www.w3.org/2001/XMLSchema");

    RsFile {
        name: "".into(),
        namespace: None,
        target_ns: target_namespace(schema).cloned(),
        xsd_ns: xsd_namespaces
            .clone()
            .find(|namespace| namespace.name().is_some())
            .or_else(|| xsd_namespaces.next())
            .cloned(),
        types: schema
            .children()
            .filter(|n| {
                n.is_element()
                    && n.xsd_type() != ElementType::Annotation
                    && n.xsd_type() != ElementType::AttributeGroup
                    && n.xsd_type() != ElementType::Group
            })
            .map(|node| node.parse(schema))
            .collect(),
        attribute_groups: schema
            .children()
            .filter(|n| n.is_element() && n.xsd_type() == ElementType::AttributeGroup)
            .map(|node| node.parse(schema))
            .collect(),
        groups: schema
            .children()
            .filter(|n| n.is_element() && n.xsd_type() == ElementType::Group)
            .map(|node| (node.attr_name().unwrap().to_string(), node.parse(schema)))
            .filter_map(|node| match node {
                (k, types::RsEntity::Group(g)) => Some((k, g)),
                _ => None,
            })
            .collect(),
    }
}

#[cfg(test)]
mod test {
    use crate::parser::schema::parse;

    #[test]
    fn test_single_xsd_ns() {
        let doc = roxmltree::Document::parse(
            r#"
    <xs:schema
        xmlns:tt="http://www.onvif.org/ver10/schema"
        xmlns:xs="http://www.w3.org/2001/XMLSchema"
        targetNamespace="http://www.onvif.org/ver10/schema"
        >
    </xs:schema>
                "#,
        )
        .unwrap();

        let res = parse(&doc.root_element());
        assert_eq!(res.xsd_ns.unwrap().name().unwrap(), "xs");
    }

    #[test]
    fn test_multiple_xsd_ns() {
        let doc = roxmltree::Document::parse(
            r#"
    <xs:schema
        xmlns:tt="http://www.onvif.org/ver10/schema"
        xmlns="http://www.w3.org/2001/XMLSchema"
        xmlns:xs="http://www.w3.org/2001/XMLSchema"
        xmlns:xsd="http://www.w3.org/2001/XMLSchema"
        targetNamespace="http://www.onvif.org/ver10/schema"
        >
    </xs:schema>
                "#,
        )
        .unwrap();

        let res = parse(&doc.root_element());
        assert_eq!(res.xsd_ns.unwrap().name().unwrap(), "xs");
    }
}
