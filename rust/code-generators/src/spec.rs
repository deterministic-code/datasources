use crate::compile_filter::types_filter;
use crate::yaml_cursor::YamlCursor;
use datasources::{DatasourceError, FieldSize, Result, SpecField};
use serde_yaml::Value;
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct SpecType {
    pub name: String,
    pub tags: Vec<String>,
    pub kind: String,
    pub inherits: Option<String>,
    pub union: Option<Vec<String>>,
    pub mapping: Option<HashMap<String, String>>,
    pub extract: Option<Vec<String>>,
    pub remove_fields: Option<Vec<String>>,
    pub ids: Option<Vec<String>>,
    pub fields: Vec<SpecTypeField>,
}

#[derive(Clone, Debug)]
pub struct SpecTypeField {
    pub name: String,
    pub type_name: String,
    pub kind: String,
    pub base: String,
    pub is_array: bool,
    pub is_map: bool,
    pub is_nullable: bool,
    pub is_id: bool,
    pub size: Option<SpecSize>,
    pub references: Option<String>,
    pub reference_pair: Option<Vec<String>>,
    pub default_value: Option<Value>,
    pub has_default: bool,
    pub source: Option<String>,
    pub original: Option<String>,
}

#[derive(Clone, Debug)]
pub enum SpecSize {
    Unlimited,
    Length(i32),
    Decimal(i32, i32),
}

impl SpecTypeField {
    pub fn converter_field(&self) -> SpecField {
        SpecField {
            type_name: self.type_name.clone(),
            name: Some(self.name.clone()),
            size: self.size.as_ref().map(|size| match size {
                SpecSize::Unlimited => FieldSize::Unlimited,
                SpecSize::Length(n) => FieldSize::Length(*n),
                SpecSize::Decimal(p, s) => FieldSize::Decimal(*p, *s),
            }),
            default_value: self.default_value.as_ref().map(yaml_text),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DatasourceFieldOverlay {
    pub name: String,
    pub is_readonly: Option<bool>,
    pub is_unique: Option<bool>,
    pub mapping: Option<String>,
    pub is_fixed_id: Option<bool>,
    pub is_optimistic_concurrency: Option<bool>,
    pub use_native_row_version: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct DatasourceIndex {
    pub name: String,
    pub fields: Vec<String>,
    pub is_unique: bool,
}

#[derive(Clone, Debug)]
pub struct DatasourceTable {
    pub name: String,
    pub mapping: Option<String>,
    pub fields: Vec<DatasourceFieldOverlay>,
    pub indexes: Vec<DatasourceIndex>,
    pub unique_index_fields: Vec<String>,
    pub use_optimistic_concurrency: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct SeedRow {
    pub id: i32,
    pub row: IndexMap<String, Value>,
}

#[derive(Clone, Debug, Default)]
pub struct ParsedSpec {
    pub types: Vec<SpecType>,
    pub expanded_types: Vec<SpecType>,
    pub datasource: Vec<DatasourceTable>,
    pub datasource_seeds: HashMap<String, Vec<SeedRow>>,
}

pub const TYPES_YAML: &str = "types.yaml";
pub const DATASOURCE_YAML: &str = "datasource.yaml";
pub const SEEDS_YAML: &str = "datasource_seeds.yaml";

const PRIMITIVES: &[&str] = &[
    "string",
    "character",
    "number",
    "integer",
    "unsignedinteger",
    "biginteger",
    "unsignedbiginteger",
    "smallinteger",
    "unsignedsmallinteger",
    "float",
    "decimal",
    "boolean",
    "datetime",
    "binary",
    "uuid",
    "reference",
];

pub fn parse_field_type(raw: &str) -> (String, String, bool, bool) {
    let is_array = raw.ends_with("[]");
    let is_map = raw.ends_with("{}");
    let base = if is_array || is_map { &raw[..raw.len() - 2] } else { raw };
    let kind = if PRIMITIVES.contains(&base) { "primitive" } else { "type" };
    (kind.into(), base.into(), is_array, is_map)
}

pub fn parse(files: &HashMap<String, String>) -> Result<ParsedSpec> {
    let spec = deterministic_generators_common::DeterministicParser::new(
        deterministic_generators_common::memory_reader(files.clone()),
    )
    .parse(&HashMap::new())
    .map_err(|error| DatasourceError::message(error.to_string()))?;
    Ok(ParsedSpec {
        types: spec.types.iter().map(map_type).collect(),
        expanded_types: spec.expanded_types.iter().map(map_type).collect(),
        datasource: spec.datasource.iter().map(map_table).collect(),
        datasource_seeds: spec
            .datasource_seeds
            .iter()
            .map(|(name, rows)| (name.clone(), rows.iter().map(map_seed).collect()))
            .collect(),
    })
}

fn map_type(type_: &deterministic_generators_common::Type) -> SpecType {
    SpecType {
        name: type_.name.clone(),
        tags: type_.tags.clone(),
        kind: match type_.kind {
            deterministic_generators_common::TypeKind::Inherit => "inherit",
            deterministic_generators_common::TypeKind::Union => "union",
            deterministic_generators_common::TypeKind::Shaped => "shaped",
        }
        .into(),
        inherits: type_.inherits.clone(),
        union: type_.union.clone(),
        mapping: type_.mapping.as_ref().map(|map| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect()),
        extract: type_.extract.clone(),
        remove_fields: type_.remove_fields.clone(),
        ids: type_.ids.clone(),
        fields: type_.fields.iter().map(map_field).collect(),
    }
}

fn map_field(field: &deterministic_generators_common::TypeField) -> SpecTypeField {
    let (references, reference_pair) = match &field.references {
        Some(deterministic_generators_common::References::One(text)) => (Some(text.clone()), None),
        Some(deterministic_generators_common::References::Pair(left, right)) => {
            (None, Some(vec![left.clone(), right.clone()]))
        }
        None => (None, None),
    };
    SpecTypeField {
        name: field.name.clone(),
        type_name: field.type_name.clone(),
        kind: match field.kind {
            deterministic_generators_common::FieldKind::Primitive => "primitive",
            deterministic_generators_common::FieldKind::Type => "type",
        }
        .into(),
        base: field.base.clone(),
        is_array: field.is_array,
        is_map: field.is_map,
        is_nullable: field.is_nullable,
        is_id: field.is_id,
        size: field.size.as_ref().map(|size| match size {
            deterministic_generators_common::FieldSize::Unlimited => SpecSize::Unlimited,
            deterministic_generators_common::FieldSize::Num(n) => SpecSize::Length(*n as i32),
            deterministic_generators_common::FieldSize::Range(p, s) => SpecSize::Decimal(*p as i32, *s as i32),
        }),
        references,
        reference_pair,
        default_value: field.default_value.as_ref().map(literal_value),
        has_default: field.has_default,
        source: None,
        original: None,
    }
}

fn map_table(table: &deterministic_generators_common::DatasourceTable) -> DatasourceTable {
    DatasourceTable {
        name: table.name.clone(),
        mapping: table.mapping.clone(),
        fields: table
            .fields
            .iter()
            .map(|field| DatasourceFieldOverlay {
                name: field.name.clone(),
                is_readonly: field.is_readonly,
                is_unique: field.is_unique,
                mapping: field.mapping.clone(),
                is_fixed_id: field.is_fixed_id,
                is_optimistic_concurrency: field.is_optimistic_concurrency,
                use_native_row_version: field.use_native_row_version,
            })
            .collect(),
        indexes: table
            .indexes
            .iter()
            .map(|index| DatasourceIndex {
                name: index.name.clone(),
                fields: index.fields.clone(),
                is_unique: index.is_unique,
            })
            .collect(),
        unique_index_fields: table.unique_index_fields.clone(),
        use_optimistic_concurrency: table.use_optimistic_concurrency,
    }
}

fn map_seed(row: &deterministic_generators_common::SeedRow) -> SeedRow {
    SeedRow {
        id: i32::try_from(row.id).unwrap_or(i32::MAX),
        row: row
            .row
            .iter()
            .map(|(key, value)| (key.clone(), seed_value(value)))
            .collect(),
    }
}

fn literal_value(value: &deterministic_generators_common::YamlLiteral) -> Value {
    match value {
        deterministic_generators_common::YamlLiteral::String(text) => Value::String(text.clone()),
        deterministic_generators_common::YamlLiteral::Number(n) => Value::Number(serde_yaml::Number::from(*n)),
        deterministic_generators_common::YamlLiteral::Bool(flag) => Value::Bool(*flag),
        deterministic_generators_common::YamlLiteral::Null => Value::Null,
        deterministic_generators_common::YamlLiteral::EmptyArray => Value::Sequence(vec![]),
        deterministic_generators_common::YamlLiteral::EmptyMap => Value::Mapping(Default::default()),
    }
}

fn seed_value(value: &deterministic_generators_common::SeedValue) -> Value {
    match value {
        deterministic_generators_common::SeedValue::String(text) => Value::String(text.clone()),
        deterministic_generators_common::SeedValue::Number(n) => Value::Number(serde_yaml::Number::from(*n)),
        deterministic_generators_common::SeedValue::Bool(flag) => Value::Bool(*flag),
        deterministic_generators_common::SeedValue::Null => Value::Null,
    }
}

fn parse_types(yaml: &str) -> Result<Vec<SpecType>> {
    YamlCursor::from_yaml(yaml)
        .named_list("types")
        .into_iter()
        .map(|(name, node)| {
            let extract = node.strings("extract");
            let remove = node.strings("remove_fields");
            let ids = node.strings("ids");
            let kind = if node.str("inherits").is_some() {
                "inherit"
            } else if matches!(node.child("union").value(), Value::Sequence(_)) {
                "union"
            } else {
                "shaped"
            };
            Ok(SpecType {
                name,
                tags: node.strings("tags"),
                kind: kind.into(),
                inherits: node.str("inherits"),
                union: matches!(node.child("union").value(), Value::Sequence(_)).then(|| node.strings("union")),
                mapping: mapping_of(&node),
                extract: (!extract.is_empty()).then_some(extract),
                remove_fields: (!remove.is_empty()).then_some(remove),
                ids: (!ids.is_empty()).then_some(ids),
                fields: node
                    .named_list("fields")
                    .into_iter()
                    .map(|(field_name, field)| read_field(&field_name, &field))
                    .collect(),
            })
        })
        .collect()
}

fn read_field(name: &str, node: &YamlCursor) -> SpecTypeField {
    let raw = node.str("type").unwrap_or_else(|| "string".into());
    let (kind, base, is_array, is_map) = parse_field_type(&raw);
    let (references, pair) = references_of(node);
    let authored = node.has("default_value");
    let default_value = if authored {
        node.literal("default_value")
    } else if is_array {
        Some(Value::Sequence(vec![]))
    } else if is_map {
        Some(Value::Mapping(Default::default()))
    } else {
        None
    };
    SpecTypeField {
        name: name.into(),
        type_name: raw,
        kind,
        base,
        is_array,
        is_map,
        is_nullable: node.bool("is_nullable"),
        is_id: node.bool("is_id"),
        size: size_of(node),
        references,
        reference_pair: pair,
        has_default: authored || default_value.is_some(),
        default_value,
        source: None,
        original: None,
    }
}

fn parse_datasource(yaml: &str, types: &[SpecType]) -> Result<Vec<DatasourceTable>> {
    let root = YamlCursor::from_yaml(yaml);
    let tables: Vec<DatasourceTable> = root
        .named_list("types")
        .into_iter()
        .map(|(name, node)| {
            let mut unique = Vec::new();
            let indexes = node
                .named_list("indexes")
                .into_iter()
                .map(|(index_name, index)| {
                    let fields = match index.child("fields").value() {
                        Value::Sequence(list) => list.iter().map(yaml_text).collect(),
                        _ => vec![],
                    };
                    let is_unique = index.bool("is_unique");
                    if is_unique && fields.len() == 1 {
                        unique.push(fields[0].clone());
                    }
                    DatasourceIndex {
                        name: index_name,
                        fields,
                        is_unique,
                    }
                })
                .collect();
            let overlays = node
                .named_list("fields")
                .into_iter()
                .map(|(field_name, field)| DatasourceFieldOverlay {
                    name: field_name,
                    is_readonly: field.has("is_readonly").then(|| field.bool("is_readonly")),
                    is_unique: field.has("is_unique").then(|| field.bool("is_unique")),
                    mapping: field.str("mapping"),
                    is_fixed_id: field.has("is_fixed_id").then(|| field.bool("is_fixed_id")),
                    is_optimistic_concurrency: field
                        .has("is_optimistic_concurrency")
                        .then(|| field.bool("is_optimistic_concurrency")),
                    use_native_row_version: field
                        .has("use_native_row_version")
                        .then(|| field.bool("use_native_row_version")),
                })
                .collect();
            DatasourceTable {
                name,
                mapping: node.str("mapping"),
                fields: overlays,
                indexes,
                unique_index_fields: unique,
                use_optimistic_concurrency: node
                    .has("use_optimistic_concurrency")
                    .then(|| node.bool("use_optimistic_concurrency")),
            }
        })
        .collect();
    let Some(filter_expr) = compile_types_filter(&root)? else {
        return Ok(tables);
    };
    let selected: HashSet<String> = types
        .iter()
        .filter(|t| filter_expr(t))
        .map(|t| t.name.clone())
        .collect();
    let overlay_names: HashSet<String> = tables.iter().map(|t| t.name.clone()).collect();
    let extras = types.iter().filter(|t| selected.contains(&t.name) && !overlay_names.contains(&t.name)).map(|t| {
        DatasourceTable {
            name: t.name.clone(),
            mapping: None,
            fields: vec![],
            indexes: vec![],
            unique_index_fields: vec![],
            use_optimistic_concurrency: None,
        }
    });
    Ok(tables
        .into_iter()
        .filter(|t| selected.contains(&t.name))
        .chain(extras)
        .collect())
}

fn compile_types_filter(root: &YamlCursor) -> Result<Option<Box<dyn Fn(&SpecType) -> bool>>> {
    for entry in root.child("includes").items() {
        if entry.child("types").map_record() {
            return Ok(Some(types_filter(entry.child("types").str("filter").as_deref())?));
        }
    }
    Ok(None)
}

fn parse_seeds(yaml: &str) -> Result<HashMap<String, Vec<SeedRow>>> {
    let mut by_table = HashMap::new();
    for (name, node) in YamlCursor::from_yaml(yaml).named_list("seeds") {
        let rows = node
            .named_items()
            .into_iter()
            .map(|(row_key, row)| {
                Ok(SeedRow {
                    id: parse_seed_key(&row_key)?,
                    row: seed_cells(&row),
                })
            })
            .collect::<Result<Vec<_>>>()?;
        by_table.insert(name, rows);
    }
    Ok(by_table)
}

fn parse_seed_key(row_key: &str) -> Result<i32> {
    if row_key.len() < 3 || !row_key.starts_with("id") {
        return Err(DatasourceError::message(format!(
            "Invalid seed row key \"{row_key}\": expected pattern /^id\\d+$/"
        )));
    }
    row_key[2..].parse::<i32>().map_err(|_| {
        DatasourceError::message(format!(
            "Invalid seed row key \"{row_key}\": expected pattern /^id\\d+$/"
        ))
    })
}

fn seed_cells(node: &YamlCursor) -> IndexMap<String, Value> {
    let mut out = IndexMap::new();
    let Some(map) = node.value().as_mapping() else {
        return out;
    };
    for (key, _) in map {
        let name = yaml_text(key);
        if let Some(value) = node.literal(&name) {
            out.insert(name, value);
        } else if node.has(&name) {
            out.insert(name, Value::Null);
        }
    }
    out
}

fn mapping_of(node: &YamlCursor) -> Option<HashMap<String, String>> {
    let child = node.child("mapping");
    let map = child.value().as_mapping()?;
    let out: HashMap<String, String> = map
        .iter()
        .filter_map(|(k, v)| match v {
            Value::String(text) => Some((yaml_text(k), text.clone())),
            _ => None,
        })
        .collect();
    (!out.is_empty()).then_some(out)
}

fn references_of(node: &YamlCursor) -> (Option<String>, Option<Vec<String>>) {
    let child = node.child("references");
    match child.value() {
        Value::String(text) => (Some(text.clone()), None),
        Value::Sequence(list) if list.len() == 2 => {
            (None, Some(list.iter().map(yaml_text).collect()))
        }
        _ => (None, None),
    }
}

fn size_of(node: &YamlCursor) -> Option<SpecSize> {
    let child = node.child("size");
    match child.value() {
        Value::String(text) if text == "unlimited" => Some(SpecSize::Unlimited),
        Value::Number(n) => n.as_i64().map(|v| SpecSize::Length(v as i32)),
        Value::Sequence(list) if list.len() == 2 => {
            let a = as_i32(&list[0])?;
            let b = as_i32(&list[1])?;
            Some(SpecSize::Decimal(a, b))
        }
        _ => None,
    }
}

fn as_i32(value: &Value) -> Option<i32> {
    match value {
        Value::Number(n) => n.as_i64().map(|v| v as i32),
        _ => None,
    }
}

pub fn yaml_text(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Bool(flag) => if *flag { "true" } else { "false" }.into(),
        Value::Number(number) => number.to_string(),
        Value::Null => String::new(),
        other => format!("{other:?}"),
    }
}

fn clone_field(field: &SpecTypeField, source: Option<String>, original: Option<String>) -> SpecTypeField {
    let mut next = field.clone();
    next.source = source;
    next.original = original;
    next
}

fn set_fields() -> Vec<SpecTypeField> {
    vec![SpecTypeField {
        name: "id".into(),
        type_name: "integer".into(),
        kind: "primitive".into(),
        base: "integer".into(),
        is_array: false,
        is_map: false,
        is_nullable: false,
        is_id: false,
        size: None,
        references: None,
        reference_pair: None,
        default_value: None,
        has_default: false,
        source: None,
        original: None,
    }]
}

pub fn expand_types(types: &[SpecType]) -> Result<Vec<SpecType>> {
    let by_name: HashMap<String, SpecType> = types.iter().map(|t| (t.name.clone(), t.clone())).collect();
    let mut cache: HashMap<String, Vec<SpecTypeField>> = HashMap::new();

    fn resolve(
        name: &str,
        by_name: &HashMap<String, SpecType>,
        cache: &mut HashMap<String, Vec<SpecTypeField>>,
        stack: &mut HashSet<String>,
    ) -> Result<Vec<SpecTypeField>> {
        if let Some(hit) = cache.get(name) {
            return Ok(hit.clone());
        }
        if name == "set" {
            cache.insert(name.into(), set_fields());
            return Ok(set_fields());
        }
        if !stack.insert(name.to_string()) {
            return Err(DatasourceError::message(format!(
                "circular inherit/union involving \"{name}\""
            )));
        }
        let Some(type_) = by_name.get(name) else {
            stack.remove(name);
            return Ok(vec![]);
        };
        let compose = |source: &str, cache: &mut HashMap<String, Vec<SpecTypeField>>, stack: &mut HashSet<String>| {
            let composed: Vec<SpecTypeField> = resolve(source, by_name, cache, stack)?
                .into_iter()
                .map(|f| clone_field(&f, Some(source.into()), Some(f.name.clone())))
                .collect();
            Ok(if by_name.get(source).and_then(|t| t.inherits.as_deref()) == Some("dictionary") {
                composed
                    .into_iter()
                    .filter(|f| !is_owner_identity_ref(f, source, by_name))
                    .collect()
            } else {
                composed
            })
        };
        let skip_set_id = type_.inherits.as_deref() == Some("set") && has_authored_identity(type_);
        let mut inherited = Vec::new();
        if let Some(parent) = &type_.inherits {
            if !skip_set_id {
                inherited.extend(compose(parent, cache, stack)?);
            }
        }
        for member in type_.union.iter().flatten() {
            inherited.extend(compose(member, cache, stack)?);
        }
        stack.remove(name);
        let mut merged = keep(rename(extract(inherited, type_.extract.as_deref()), type_.mapping.as_ref()), type_.remove_fields.as_deref());
        merged.extend(type_.fields.iter().map(|f| clone_field(f, Some(name.into()), Some(f.name.clone()))));
        assert_unique(&merged, name)?;
        let result: Vec<SpecTypeField> = merged.into_iter().map(|f| clone_field(&f, None, None)).collect();
        cache.insert(name.into(), result.clone());
        Ok(result)
    }

    fn resolve_ids(
        name: &str,
        by_name: &HashMap<String, SpecType>,
        stack: &mut HashSet<String>,
    ) -> Option<Vec<String>> {
        if !stack.insert(name.to_string()) {
            return None;
        }
        let type_ = by_name.get(name)?;
        if let Some(ids) = &type_.ids {
            if !ids.is_empty() {
                return Some(ids.clone());
            }
        }
        if type_.fields.iter().any(|f| f.is_id) {
            return None;
        }
        match type_.inherits.as_deref() {
            Some(parent) if parent != "set" && parent != "dictionary" => resolve_ids(parent, by_name, stack),
            _ => None,
        }
    }

    types
        .iter()
        .map(|type_| {
            let ids = type_.ids.clone().or_else(|| resolve_ids(&type_.name, &by_name, &mut HashSet::new()));
            Ok(SpecType {
                name: type_.name.clone(),
                tags: type_.tags.clone(),
                kind: type_.kind.clone(),
                inherits: type_.inherits.clone(),
                union: type_.union.clone(),
                mapping: type_.mapping.clone(),
                extract: type_.extract.clone(),
                remove_fields: type_.remove_fields.clone(),
                ids: if ids.as_ref().map(|v| !v.is_empty()).unwrap_or(false) {
                    ids
                } else {
                    type_.ids.clone()
                },
                fields: resolve(&type_.name, &by_name, &mut cache, &mut HashSet::new())?,
            })
        })
        .collect()
}

fn extract(fields: Vec<SpecTypeField>, extract: Option<&[String]>) -> Vec<SpecTypeField> {
    let Some(extract) = extract.filter(|e| !e.is_empty()) else {
        return fields;
    };
    let mut allow: HashMap<String, HashSet<String>> = HashMap::new();
    for item in extract {
        if let Some((left, right)) = split_dot(item) {
            allow.entry(left).or_default().insert(right);
        }
    }
    fields
        .into_iter()
        .filter(|f| {
            allow
                .get(f.source.as_deref().unwrap_or(""))
                .map(|keep| keep.contains(&f.name))
                .unwrap_or(true)
        })
        .collect()
}

fn rename(fields: Vec<SpecTypeField>, mapping: Option<&HashMap<String, String>>) -> Vec<SpecTypeField> {
    let Some(mapping) = mapping else {
        return fields;
    };
    fields
        .into_iter()
        .map(|mut f| {
            let key = format!("{}.{}", f.source.as_deref().unwrap_or(""), f.name);
            if let Some(name) = mapping.get(&key).or_else(|| mapping.get(&f.name)) {
                f.name = name.clone();
            }
            f
        })
        .collect()
}

fn keep(fields: Vec<SpecTypeField>, remove: Option<&[String]>) -> Vec<SpecTypeField> {
    let Some(remove) = remove.filter(|e| !e.is_empty()) else {
        return fields;
    };
    fields
        .into_iter()
        .filter(|f| {
            !remove.iter().any(|item| match split_dot(item) {
                Some((left, right)) => f.name == right && f.source.as_deref() == Some(left.as_str()),
                None => f.name == *item,
            })
        })
        .collect()
}

fn assert_unique(fields: &[SpecTypeField], type_name: &str) -> Result<()> {
    let mut seen: HashMap<String, &SpecTypeField> = HashMap::new();
    for field in fields {
        if let Some(prev) = seen.get(&field.name) {
            return Err(DatasourceError::message(format!(
                "{}.{} and {}.{} collide on the composed shape of {type_name}; use mapping or remove_fields and give new names",
                prev.source.as_deref().unwrap_or(""),
                prev.original.as_deref().unwrap_or(""),
                field.source.as_deref().unwrap_or(""),
                field.original.as_deref().unwrap_or("")
            )));
        }
        seen.insert(field.name.clone(), field);
    }
    Ok(())
}

pub fn type_has_tag(type_: &SpecType, tag: &str) -> bool {
    type_.tags.iter().any(|t| t == tag)
}

pub fn datasource_types_of(spec: &ParsedSpec) -> Vec<SpecType> {
    spec.expanded_types
        .iter()
        .filter(|type_| type_has_tag(type_, "datasource_type"))
        .cloned()
        .collect()
}

pub fn is_dictionary_type(type_: Option<&SpecType>) -> bool {
    type_.and_then(|t| t.inherits.as_deref()) == Some("dictionary")
}

pub fn is_collection_field(field: &SpecTypeField) -> bool {
    field.kind == "type" && (field.is_array || field.is_map)
}

pub fn column_fields(fields: impl IntoIterator<Item = SpecTypeField>) -> Vec<SpecTypeField> {
    fields.into_iter().filter(|field| !is_collection_field(field)).collect()
}

pub fn has_authored_identity(type_: &SpecType) -> bool {
    type_.ids.as_ref().map(|ids| !ids.is_empty()).unwrap_or(false) || type_.fields.iter().any(|f| f.is_id)
}

pub fn identity_columns(type_: Option<&SpecType>, table: Option<&DatasourceTable>) -> Vec<String> {
    if let Some(ids) = type_.and_then(|t| t.ids.as_ref()).filter(|ids| !ids.is_empty()) {
        return ids.clone();
    }
    let marked: Vec<String> = type_
        .map(|t| t.fields.iter().filter(|f| f.is_id).map(|f| f.name.clone()).collect())
        .unwrap_or_default();
    if !marked.is_empty() {
        return marked;
    }
    if let Some(fixed) = table.and_then(|t| t.fields.iter().find(|f| f.is_fixed_id == Some(true))) {
        return vec![fixed.name.clone()];
    }
    if type_.and_then(|t| t.inherits.as_deref()) == Some("set") || type_.is_none() {
        vec!["id".into()]
    } else {
        vec![]
    }
}

pub fn pk_name(type_: &SpecType, table: Option<&DatasourceTable>) -> String {
    identity_columns(Some(type_), table).into_iter().next().unwrap_or_else(|| "id".into())
}

pub fn is_pk_field(field: &SpecTypeField, type_: &SpecType, table: Option<&DatasourceTable>) -> bool {
    identity_columns(Some(type_), table).iter().any(|name| name == &field.name)
}

pub fn unique_lookup_fields(type_: &SpecType, table: Option<&DatasourceTable>) -> Vec<String> {
    let mut names = Vec::new();
    let mut add = |name: String| {
        if names.contains(&name) {
            return;
        }
        if let Some(field) = type_.fields.iter().find(|f| f.name == name) {
            if is_collection_field(field) {
                return;
            }
        }
        names.push(name);
    };
    for name in identity_columns(Some(type_), table) {
        add(name);
    }
    if let Some(table) = table {
        for overlay in &table.fields {
            if overlay.is_unique == Some(true) {
                add(overlay.name.clone());
            }
        }
        for name in &table.unique_index_fields {
            add(name.clone());
        }
    }
    names
}

pub fn table_by_name(spec: &ParsedSpec) -> HashMap<String, DatasourceTable> {
    spec.datasource.iter().map(|table| (table.name.clone(), table.clone())).collect()
}

pub fn reference_target(field: &SpecTypeField) -> (String, String) {
    if let Some(pair) = field.reference_pair.as_ref().filter(|p| p.len() == 2) {
        return (pair[0].clone(), pair[1].clone());
    }
    let parts: Vec<&str> = field.references.as_deref().unwrap_or("").splitn(2, '.').collect();
    (
        parts.first().copied().unwrap_or("").into(),
        parts.get(1).copied().unwrap_or("").into(),
    )
}

pub fn split_dot(value: &str) -> Option<(String, String)> {
    value.find('.').map(|i| (value[..i].into(), value[i + 1..].into()))
}

pub fn is_owner_identity_ref(field: &SpecTypeField, self_name: &str, by_name: &HashMap<String, SpecType>) -> bool {
    let refs: Vec<String> = field
        .reference_pair
        .clone()
        .or_else(|| field.references.clone().map(|one| vec![one]))
        .unwrap_or_default();
    if refs.is_empty() {
        return false;
    }
    let parts: Vec<(String, String)> = refs
        .iter()
        .map(|r| split_dot(r).unwrap_or_else(|| (String::new(), r.clone())))
        .collect();
    let owner = &parts[0].0;
    if owner.is_empty() || owner == self_name {
        return false;
    }
    let identity = identity_names_of(by_name.get(owner), by_name, &mut HashSet::new());
    parts.iter().all(|p| p.0 == *owner && identity.contains(&p.1))
}

fn identity_names_of(
    type_: Option<&SpecType>,
    by_name: &HashMap<String, SpecType>,
    stack: &mut HashSet<String>,
) -> HashSet<String> {
    let Some(type_) = type_ else {
        return HashSet::new();
    };
    if !stack.insert(type_.name.clone()) {
        return HashSet::new();
    }
    let local = identity_columns(Some(type_), None);
    if !local.is_empty() {
        return local.into_iter().collect();
    }
    match type_.inherits.as_deref() {
        Some(parent) if parent != "dictionary" => identity_names_of(by_name.get(parent), by_name, stack),
        _ => HashSet::new(),
    }
}

pub fn persisted_column_fields(type_: &SpecType, by_name: &HashMap<String, SpecType>) -> Vec<SpecTypeField> {
    if !is_dictionary_type(Some(type_)) {
        return column_fields(type_.fields.clone());
    }
    let Some(value) = type_.fields.iter().find(|f| f.name == "value") else {
        return column_fields(type_.fields.clone());
    };
    if value.kind != "type" || value.is_array || value.is_map {
        return column_fields(type_.fields.clone());
    }
    let Some(nested) = by_name.get(&value.base) else {
        return column_fields(type_.fields.clone());
    };
    let mut cols = column_fields(type_.fields.iter().filter(|f| f.name != "value").cloned());
    cols.extend(column_fields(nested.fields.clone()));
    cols
}

pub fn dictionary_unique_columns(type_: &SpecType, by_name: &HashMap<String, SpecType>) -> Vec<String> {
    if !is_dictionary_type(Some(type_)) {
        return vec![];
    }
    let mut owners: Vec<String> = type_
        .fields
        .iter()
        .filter(|f| is_owner_identity_ref(f, &type_.name, by_name))
        .map(|f| f.name.clone())
        .collect();
    if type_.fields.iter().any(|f| f.name == "key") {
        owners.push("key".into());
    }
    owners
}
