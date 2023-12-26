use super::*;
use argh::argh;
use std::collections::HashMap;

#[test]
fn test_argh_with_value_without_custom_splitter() {
    let mut args = vec![
        "path/to/target/".to_string(),
        "key1:value1".to_string(),
        "key2:value2".to_string(),
        "key4:abc".to_string(),
    ];

    let result = argh(&mut args, None);
    let expected : HashMap<&str, Option<&str>> = [
        ("key1",Some("value1")),
        ("key2",Some("value2")),
        ("key4",Some("abc")),
    ]
        .iter()
        .cloned()
        .collect();

    assert_eq!(result, expected);
}

#[test]
fn test_argh_without_value_without_custom_splitter() {
    let mut args = vec![
        "path/to/target/".to_string(),
        "key2".to_string(),
        "key3".to_string(),
    ];

    let result = argh(&mut args, None);
    let expected : HashMap<&str, Option<&str>> = [
        ("key2", None),
        ("key3", None)
    ]
        .iter()
        .cloned()
        .collect();

    assert_eq!(result, expected)
}

#[test]
fn test_argh_with_value_with_custom_splitter() {
    let mut args = vec![
        "path/to/target/".to_string(),
        "key1|value1".to_string(),
        "key2|value2".to_string(),
        "key3".to_string(),
        "key4|abc".to_string(),
    ];

    let result = argh(&mut args, Some("|"));
    let expected : HashMap<&str, Option<&str>> = [
        ("key1",Some("value1")),
        ("key2",Some("value2")),
        ("key3",None),
        ("key4",Some("abc")),
    ]
        .iter()
        .cloned()
        .collect();

    assert_eq!(result, expected);
}
