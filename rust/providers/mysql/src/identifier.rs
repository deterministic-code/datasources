use datasources::{
    CheckConstraint, CreateTableStatementTextRenderer, ForeignKey, PrimaryKey, TableConstraint,
    UniqueConstraint,
};

/// MySQL identifiers are at most 64 characters.
const LIMIT: usize = 64;

const ROLES: &[&str] = &[
    "default_constraint",
    "unique_constraint",
    "foreign_key",
    "primary_key",
    "updated_at",
    "index",
];

/// Leave a name that fits. A longer one keeps its role suffix and a hash of the
/// full name, so create and drop statements spell it the same way.
pub fn fit(name: &str) -> String {
    if name.chars().count() <= LIMIT {
        return name.to_string();
    }
    let suffix = preserved_suffix(name);
    let sep = if suffix.starts_with('-') { "-" } else { "_" };
    let tail = format!("{sep}{}{suffix}", hash(name));
    let head: String = name.chars().take(LIMIT - tail.chars().count()).collect();
    format!("{}{tail}", head.trim_end_matches(['_', '-']))
}

pub fn render_constraint<T>(renderer: &T, constraint: &TableConstraint) -> String
where
    T: CreateTableStatementTextRenderer + ?Sized,
{
    struct Proxy<'a, T: ?Sized>(&'a T);
    impl<T: CreateTableStatementTextRenderer + ?Sized> CreateTableStatementTextRenderer
        for Proxy<'_, T>
    {
        fn quote(&self, name: &str) -> String {
            self.0.quote(name)
        }
    }
    Proxy(renderer).constraint(&fit_constraint(constraint))
}

fn fit_constraint(constraint: &TableConstraint) -> TableConstraint {
    match constraint {
        TableConstraint::PrimaryKey(key) => TableConstraint::PrimaryKey(PrimaryKey {
            name: fit(&key.name),
            columns: key.columns.clone(),
        }),
        TableConstraint::ForeignKey(key) => TableConstraint::ForeignKey(ForeignKey {
            name: fit(&key.name),
            columns: key.columns.clone(),
            references_table: key.references_table.clone(),
            references_columns: key.references_columns.clone(),
            on_delete: key.on_delete,
            on_update: key.on_update,
        }),
        TableConstraint::Unique(key) => TableConstraint::Unique(UniqueConstraint {
            name: fit(&key.name),
            columns: key.columns.clone(),
        }),
        TableConstraint::Check(key) => TableConstraint::Check(CheckConstraint {
            name: fit(&key.name),
            expression: key.expression.clone(),
        }),
    }
}

fn preserved_suffix(name: &str) -> String {
    let lower = name.to_ascii_lowercase();
    for role in ROLES {
        let snake = format!("_{role}");
        let kebab = format!("-{}", role.replace('_', "-"));
        if lower.ends_with(&snake) {
            return name[name.len() - snake.len()..].to_string();
        }
        if lower.ends_with(&kebab) {
            return name[name.len() - kebab.len()..].to_string();
        }
    }
    for role in ROLES {
        let shaped = pascal(role);
        if name.ends_with(&shaped) {
            return name[name.len() - shaped.len()..].to_string();
        }
    }
    String::new()
}

fn pascal(role: &str) -> String {
    role.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect()
}

/// First 6 hex digits of FNV-1a 32-bit. The same function in TypeScript and C#.
fn hash(name: &str) -> String {
    let mut hash: u32 = 0x811c_9dc5;
    for byte in name.as_bytes() {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(0x0100_0193);
    }
    format!("{hash:08x}")[..6].to_string()
}

#[cfg(test)]
mod tests {
    use super::fit;

    #[test]
    fn leaves_a_name_that_already_fits() {
        let name = "a".repeat(64);
        assert_eq!(fit(&name), name);
        assert_eq!(fit("users_email_index"), "users_email_index");
    }

    #[test]
    fn truncates_a_long_name_and_keeps_a_hash_of_the_full_name() {
        assert_eq!(fit(&"a".repeat(65)), format!("{}_2dd603", "a".repeat(57)));
        assert_eq!(
            fit("game_journey_settings_game_journey_setting_category_type_id_index"),
            "game_journey_settings_game_journey_setting_category_d88931_index"
        );
        assert_eq!(
            fit("trg_game_journey_setting_category_preference_option_values_updated_at"),
            "trg_game_journey_setting_category_preference_o_8e15ab_updated_at"
        );
        assert_eq!(
            fit("notification-types-channel-name-and-something-very-long-foreign-key"),
            "notification-types-channel-name-and-something-61b8c2-foreign-key"
        );
        assert_eq!(
            fit(&format!("{}_{}", "a".repeat(56), "b".repeat(9))),
            format!("{}_a955d9", "a".repeat(56))
        );
        assert_eq!(
            fit("GameJourneySettingsGameJourneySettingCategoryTypeIdentifierForeignKey"),
            "GameJourneySettingsGameJourneySettingCategoryTy_aa3be2ForeignKey"
        );
        assert_eq!(
            fit("GAME_JOURNEY_SETTINGS_GAME_JOURNEY_SETTING_CATEGORY_TYPE_ID_INDEX"),
            "GAME_JOURNEY_SETTINGS_GAME_JOURNEY_SETTING_CATEGORY_3bfc58_INDEX"
        );
    }
}
