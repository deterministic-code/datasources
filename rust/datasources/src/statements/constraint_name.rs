use super::statement_naming::StatementNaming;

pub fn constraint_name_of(entity: &str, naming: &StatementNaming, parts: &[&str]) -> String {
    if naming.is_verbatim(entity) {
        let mut bits = vec![entity.to_string()];
        bits.extend(parts.iter().map(|p| (*p).to_string()));
        bits.join("_")
    } else {
        let mut bits = vec![naming.physical_stem(entity)];
        bits.extend(parts.iter().map(|p| (*p).to_string()));
        naming.letter(&naming.convert(&bits.join("_"), naming.constraint_name_casing))
    }
}
