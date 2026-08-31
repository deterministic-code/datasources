use crate::converters::{
    Casing, ChangeCasingConverter, PluralizerConverter, RegexConverter, ReplaceConverter,
    RowTypeConverter, ToLowerCaseConverter, ToUpperCaseConverter, TypeConverter,
};
use crate::result::Result;
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
    let converter = RowTypeConverter::new(vec!["id".into(), "name".into()]);
    let row = record(&[("id", Value::Int(7)), ("name", Value::from("ada"))]);
    let dest = converter.convert(&row).unwrap();
    assert_eq!(dest, row);
    assert_eq!(converter.reverse_convert(&dest).unwrap(), row);
}

struct OnlyOne;

impl TypeConverter<Value, Value> for OnlyOne {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(match from {
            Value::Int(number) => Value::Bool(*number == 1),
            other => other.clone(),
        })
    }

    fn reverse_convert(&self, to: &Value) -> Result<Value> {
        Ok(to.clone())
    }
}

#[test]
fn field_override_beats_type_default() {
    let converter = RowTypeConverter::new(vec!["is_enabled".into()]).with_fields(HashMap::from([
        (
            "is_enabled".into(),
            Box::new(OnlyOne) as Box<dyn TypeConverter<Value, Value> + Send + Sync>,
        ),
    ]));
    let row = record(&[("is_enabled", Value::Int(2))]);
    assert_eq!(
        converter.convert(&row).unwrap()["is_enabled"],
        Value::Bool(false)
    );
    assert_eq!(
        converter
            .convert(&record(&[("is_enabled", Value::Int(1))]))
            .unwrap()["is_enabled"],
        Value::Bool(true)
    );
}

#[test]
fn regex_and_replace() {
    assert_eq!(
        RegexConverter::new(r"\d+", "").convert("Ada 1 Lovelace"),
        "Ada  Lovelace"
    );
    assert_eq!(RegexConverter::new(r"id_(\d+)", "id-$1").convert("id_7"), "id-7");
    assert_eq!(
        RegexConverter::new(r"id_(\d+)", "id-$1").reverse_convert("id_7".into()),
        "id_7"
    );
    assert_eq!(
        ReplaceConverter::new("_", " ").convert("Ada_Lovelace"),
        "Ada Lovelace"
    );
    assert_eq!(
        ReplaceConverter::new("_", " ").reverse_convert("Ada Lovelace".into()),
        "Ada_Lovelace"
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
