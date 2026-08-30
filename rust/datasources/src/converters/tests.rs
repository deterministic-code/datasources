use crate::converters::{
    Casing, ChangeCasingConverter, PluralizerConverter, RowConverter, ToLowerCaseConverter,
    ToUpperCaseConverter, TypeConverter, ValueConverter,
};
use crate::row::Row;
use crate::value::Value;
use std::collections::HashMap;

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(k, v)| ((*k).to_string(), v.clone()))
        .collect()
}

#[test]
fn type_converter_round_trips_rows() {
    let converter = TypeConverter::new(vec!["id".into(), "name".into()]);
    let row = record(&[("id", Value::Int(7)), ("name", Value::from("ada"))]);
    let dest = converter.convert(&row).unwrap();
    assert_eq!(dest, row);
    assert_eq!(converter.reverse_convert(&dest).unwrap(), row);
    assert_eq!(converter.fields, vec!["id", "name"]);
}

#[test]
fn row_converter_clones() {
    let rows = RowConverter::default();
    let row = record(&[("display_name", Value::from("a"))]);
    assert_eq!(rows.convert(&row).unwrap(), row);
    assert_eq!(rows.reverse_convert(&row).unwrap(), row);
}

#[test]
fn field_override_beats_type_default() {
    let only_one = ValueConverter::new(
        |value| {
            Ok(match value {
                Value::Int(number) => Value::Bool(*number == 1),
                other => other.clone(),
            })
        },
        |value| Ok(value.clone()),
    );
    let schema = HashMap::from([("is_enabled".into(), "bool".into())]);
    let converter = TypeConverter::new(vec!["is_enabled".into()])
        .with_field_types(schema.clone())
        .with_fields(HashMap::from([("is_enabled".into(), only_one)]));
    let by_type = TypeConverter::new(vec!["is_enabled".into()]).with_field_types(schema.clone());
    let row = record(&[("is_enabled", Value::Int(2))]);
    assert_eq!(by_type.convert(&row).unwrap()["is_enabled"], Value::Bool(true));
    assert_eq!(
        converter.convert(&row).unwrap()["is_enabled"],
        Value::Bool(false)
    );
    let typed = TypeConverter::new(vec!["is_enabled".into()]).with_field_types(schema);
    assert_eq!(
        typed.convert(&record(&[("is_enabled", Value::from("true"))])).unwrap()["is_enabled"],
        Value::Bool(true)
    );
    assert_eq!(
        typed.convert(&record(&[("is_enabled", Value::Null)])).unwrap()["is_enabled"],
        Value::Null
    );
    assert_eq!(
        typed.convert(&record(&[("is_enabled", Value::Float(1.5))])).unwrap()["is_enabled"],
        Value::Float(1.5)
    );
}

#[test]
fn to_lower_and_to_upper_case() {
    assert_eq!(ToLowerCaseConverter.convert("ADA"), "ada");
    assert_eq!(ToLowerCaseConverter.reverse_convert("ADA".into()), "ADA");
    assert_eq!(ToUpperCaseConverter.convert("ada"), "ADA");
    assert_eq!(ToUpperCaseConverter.reverse_convert("ada".into()), "ada");
}

#[test]
fn change_casing() {
    let forms = [
        (Casing::Camel, "twoWords"),
        (Casing::Capital, "Two Words"),
        (Casing::Constant, "TWO_WORDS"),
        (Casing::Dot, "two.words"),
        (Casing::Kebab, "two-words"),
        (Casing::No, "two words"),
        (Casing::Pascal, "TwoWords"),
        (Casing::PascalSnake, "Two_Words"),
        (Casing::Path, "two/words"),
        (Casing::Sentence, "Two words"),
        (Casing::Snake, "two_words"),
        (Casing::Train, "Two-Words"),
    ];
    for (_, from) in forms {
        for (casing, expected) in forms {
            assert_eq!(ChangeCasingConverter::new(casing).convert(from), expected);
        }
        assert_eq!(
            ChangeCasingConverter::new(Casing::Snake).reverse_convert(from.into()),
            from
        );
    }
    assert_eq!(
        ChangeCasingConverter::new(Casing::Camel).convert("XMLParser"),
        "xmlParser"
    );
    assert_eq!(
        ChangeCasingConverter::new(Casing::Camel).convert("html5"),
        "html5"
    );
    assert_eq!(
        ChangeCasingConverter::new(Casing::Camel).convert("v2Name"),
        "v2Name"
    );
    assert_eq!(ChangeCasingConverter::new(Casing::Snake).convert(""), "");
    assert_eq!(ChangeCasingConverter::new(Casing::Snake).convert("___"), "");
    assert_eq!(
        ChangeCasingConverter::new(Casing::Snake).convert(" two  words\t"),
        "two_words"
    );
    assert_eq!(ChangeCasingConverter::new(Casing::Camel).convert(""), "");
}

#[test]
fn pluralizes_and_singularizes() {
    let pairs = [
        ("person", "people"),
        ("child", "children"),
        ("man", "men"),
        ("woman", "women"),
        ("mouse", "mice"),
        ("goose", "geese"),
        ("tooth", "teeth"),
        ("foot", "feet"),
        ("ox", "oxen"),
        ("sheep", "sheep"),
        ("fish", "fish"),
        ("series", "series"),
        ("species", "species"),
        ("deer", "deer"),
        ("news", "news"),
        ("box", "boxes"),
        ("dish", "dishes"),
        ("church", "churches"),
        ("quiz", "quizes"),
        ("kiss", "kisses"),
        ("bus", "buses"),
        ("city", "cities"),
        ("cat", "cats"),
        ("toy", "toys"),
        ("2y", "2ys"),
        ("Person", "People"),
        ("", ""),
    ];
    for (singular, plural) in pairs {
        assert_eq!(PluralizerConverter.convert(singular), plural);
        assert_eq!(PluralizerConverter.convert(plural), plural);
        assert_eq!(PluralizerConverter.reverse_convert(plural), singular);
        assert_eq!(PluralizerConverter.reverse_convert(singular), singular);
    }
}
