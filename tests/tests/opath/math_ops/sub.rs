use std::f64;

use super::*;

#[test]
fn integer_integer() {
    let results = query("2-3", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.data().is_integer());
    assert_eq!(res.as_integer().unwrap(), -1);

    let results = query("-2-3", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.data().is_integer());
    assert_eq!(res.as_integer().unwrap(), -5);
}

#[test]
fn integer_float() {
    let results = query("3 - 2.1", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.data().is_float());
    assert!(res.data().is_float());
    assert_approx_eq!(res.as_float(), 0.9)
}

#[test]
fn integer_string() {
    let results = query("3 - '2'", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_integer().unwrap(), 1);
}

#[test]
fn integer_undefined() {
    let results = query("2 - undefined", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn integer_bool() {
    let results = query("2 - true", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), 1.0);
}

#[test]
fn integer_null() {
    let results = query("1 - null", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), 1.0);
}

#[test]
fn integer_array() {
    let results = query("2 - @.array", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn integer_empty_array() {
    let results = query("2 - @.empty_array", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), 2.0);
}

#[test]
fn integer_object() {
    let results = query("2 - @.empty_object", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn float_float() {
    let results = query("3.1 - 2.1", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.data().is_float());
    assert_approx_eq!(res.as_float(), 1.0);

    let results = query("-2.1 - 3.1", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.data().is_float());
    assert_approx_eq!(res.as_float(), -5.2);
}

#[test]
fn float_integer() {
    let results = query("3.1 - 2", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert_approx_eq!(res.as_float(), 1.1);
}

#[test]
fn float_string() {
    let results = query("3.1 - '2'", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_approx_eq!(res.as_float(), 1.1);
}

#[test]
fn float_undefined() {
    let results = query("2.1 - undefined", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn float_bool() {
    let results = query("2.1 - true", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert_approx_eq!(res.as_float(), 1.1);
}

#[test]
fn float_array() {
    let results = query("2.1 - @.array", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn float_object() {
    let results = query("2.1 - @.empty_object", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn float_null() {
    let results = query("1.1 - null", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_approx_eq!(res.as_float(), 1.1);
}

#[test]
fn bool_integer() {
    let results = query("false - 2", EXAMPLE_JSON);
    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert_eq!(res.as_float(), -2.0);
}

#[test]
fn bool_float() {
    let results = query("false - 2.1", EXAMPLE_JSON);
    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert_eq!(res.as_float(), -2.1);
}

#[test]
fn bool_null() {
    let results = query("false - null", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), 0.0);
}

#[test]
fn bool_string() {
    let results = query("false - 'string'", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn bool_array() {
    let results = query("false - @.array", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn bool_object() {
    let results = query("false - @.empty_object", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn bool_undefined() {
    let results = query("false - undefined", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn bool_bool() {
    let results = query("true - true", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), 0.0);

    let results = query("true - false", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), 1.0);

    let results = query("false - true", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), -1.0);
}

#[test]
fn null_bool() {
    let results = query("null - true", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), -1.0);
}

#[test]
fn null_string() {
    let results = query("null - 'aaa'", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn null_float() {
    let results = query("null - 1.1", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_approx_eq!(res.as_float(), -1.1);
}

#[test]
fn null_integer() {
    let results = query("null - 1", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), -1.0);
}

#[test]
fn null_array() {
    let results = query("null - @.array ", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn null_object() {
    let results = query("null - @.empty_object", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn null_undefined() {
    let results = query("null - undefined", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn null_null() {
    let results = query("null - null", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), 0.0);
}

#[test]
fn object_null() {
    let results = query("@.empty_object - null", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), -0.0);
}

#[test]
fn object_integer() {
    let results = query("@.empty_object - 1", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), -1.0);
}

#[test]
fn object_float() {
    let results = query("@.empty_object - 1.1", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), -1.1);
}

#[test]
fn object_string() {
    let results = query("@.empty_object - 'string'", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn object_object() {
    let results = query("@.empty_object - @.empty_object", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn object_undefined() {
    let results = query("@.empty_object - undefined", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn object_empty_array() {
    let results = query("@.empty_object - @.empty_array", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), -0.0);
}

#[test]
fn object_array() {
    let results = query("@.empty_object - @.array", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn object_bool() {
    let results = query("@.empty_object - true", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), -1.0);
}

#[test]
fn array_object() {
    let results = query("@.array - @.empty_object ", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn array_null() {
    let results = query("@.array - null", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn empty_array_null() {
    let results = query("@.empty_array - null", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), 0.0);
}

#[test]
fn array_bool() {
    let results = query("@.array - true", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn empty_array_bool() {
    let results = query("@.empty_array - true", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), -1.0);
}

#[test]
fn array_integer() {
    let results = query("@.array - 2", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn empty_array_integer() {
    let results = query("@.empty_array - 2", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), -2.0);
}

#[test]
fn array_float() {
    let results = query("@.array - 2.1", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn empty_array_float() {
    let results = query("@.empty_array - 2.1", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), -2.1);
}

#[test]
fn array_string() {
    let results = query("@.array - 'string'", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn array_array() {
    let results = query("@.array - @.array", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn empty_array_empty_array() {
    let results = query("@.empty_array - @.empty_array", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), 0.0);
}

#[test]
fn array_undefined() {
    let results = query("@.array - undefined", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn undefined_float() {
    let results = query("undefined - 2.1", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn undefined_null() {
    let results = query("undefined - null", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn undefined_bool() {
    let results = query("undefined - true", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn undefined_string() {
    let results = query("undefined - 'string'", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn undefined_array() {
    let results = query("undefined - @.array", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn undefined_object() {
    let results = query("undefined - @.empty_object", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn undefined_undefined() {
    let results = query("undefined - undefined", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn undefined_integer() {
    let results = query("undefined - 2", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn number_string_null() {
    let results = query("'3' - null", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert_eq!(res.as_float(), 3.0);
}

#[test]
fn string_null() {
    let results = query("'3aaa' - null", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn number_string_bool() {
    let results = query("'3' - true", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert_eq!(res.as_float(), 2.0);
}

#[test]
fn string_bool() {
    let results = query("'3aaa' - true", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn number_string_float() {
    let results = query("'3' - 2.1", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_approx_eq!(res.as_float(), 0.9);
}

#[test]
fn string_float() {
    let results = query("'3aaa' - 2.1", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn number_string_integer() {
    let results = query("'3' - 2", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), 1.0);
}

#[test]
fn string_integer() {
    let results = query("'3aaa' - 2", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn number_string_array() {
    let results = query("'3' - @.array", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn number_string_empty_array() {
    let results = query("'3' - @.empty_array", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), 3.0);
}

#[test]
fn string_empty_array() {
    let results = query("'3aaa' - @.empty_array", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn string_object() {
    let results = query("'3' - @.empty_object", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn string_undefined() {
    let results = query("'2' - undefined", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_float());
    assert!(f64::is_nan(res.as_float()));
}

#[test]
fn string_string() {
    let results = query("'2' - '2'", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_number());
    assert_eq!(res.as_float(), 0.0);
}
