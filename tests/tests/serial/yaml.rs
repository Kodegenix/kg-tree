use crate::serial::yaml::YamlParseErrDetail;
use crate::serial::yaml::YamlParser as Parser;
use crate::tests::NodeRefExt;
use kg_tree::NodeRef;

macro_rules! assert_err {
    ($err: expr, $variant: pat) => {
        let detail = $err
            .detail()
            .downcast_ref::<YamlParseErrDetail>()
            .expect("cannot downcast to YamlParseErrDetail");

        match detail {
            $variant => {}
            err => panic!("Expected error {} got {:?}", stringify!($variant), err),
        }
    };
}

#[test]
fn null() {
    let input = r#"null"#;
    let node: NodeRef = parse_node!(input);

    assert!(node.is_null());
}

#[test]
fn boolean_false() {
    let input = r#"false"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(false, node.as_bool_ext());
}

#[test]
fn integer() {
    let input = r#"1"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(1, node.as_int_ext());
}

#[test]
fn float() {
    let input = r#"15.21"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(15.21, node.as_float_ext());
}