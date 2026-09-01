const UNCOUNTABLE: &[&str] = &["sheep", "fish", "series", "species", "deer", "news"];
const IRREGULAR: &[(&str, &str)] = &[
    ("person", "people"),
    ("child", "children"),
    ("man", "men"),
    ("woman", "women"),
    ("mouse", "mice"),
    ("goose", "geese"),
    ("tooth", "teeth"),
    ("foot", "feet"),
    ("ox", "oxen"),
];

pub struct PluralizerConverter;

impl PluralizerConverter {
    pub fn convert(&self, from: &str) -> String {
        apply(from, plural)
    }

    pub fn reverse_convert(&self, to: &str) -> String {
        apply(to, singular)
    }
}

fn apply(word: &str, transform: fn(&str) -> String) -> String {
    match word.is_empty() {
        true => String::new(),
        false => restore(word, &transform(&word.to_lowercase())),
    }
}

fn restore(original: &str, result: &str) -> String {
    match original.chars().next() {
        Some(first) if first.is_uppercase() => {
            let mut chars = result.chars();
            chars.next().unwrap().to_uppercase().collect::<String>() + chars.as_str()
        }
        _ => result.to_string(),
    }
}

fn irregular_plural(word: &str) -> Option<&'static str> {
    IRREGULAR.iter().find(|(singular, _)| *singular == word).map(|(_, plural)| *plural)
}

fn irregular_singular(word: &str) -> Option<&'static str> {
    IRREGULAR.iter().find(|(_, plural)| *plural == word).map(|(singular, _)| *singular)
}

fn plural(word: &str) -> String {
    match irregular_plural(word) {
        Some(plural) => plural.to_string(),
        None if UNCOUNTABLE.contains(&word) || irregular_singular(word).is_some() => word.to_string(),
        None if yies(word) || sibilant_es(word) => word.to_string(),
        None if sibilant(word) => format!("{word}es"),
        None if consonant_y(word) => format!("{}ies", &word[..word.len() - 1]),
        None if word.ends_with('s') => word.to_string(),
        None => format!("{word}s"),
    }
}

fn singular(word: &str) -> String {
    match irregular_singular(word) {
        Some(singular) => singular.to_string(),
        None if UNCOUNTABLE.contains(&word) || irregular_plural(word).is_some() => word.to_string(),
        None if yies(word) => format!("{}y", &word[..word.len() - 3]),
        None if sibilant_es(word) => word[..word.len() - 2].to_string(),
        None if word.ends_with('s') && !sibilant(word) => word[..word.len() - 1].to_string(),
        None => word.to_string(),
    }
}

fn yies(word: &str) -> bool {
    word.len() >= 4 && word.ends_with("ies") && consonant(nth_last(word, 4))
}

fn consonant_y(word: &str) -> bool {
    word.len() >= 2 && word.ends_with('y') && consonant(nth_last(word, 2))
}

fn sibilant_es(word: &str) -> bool {
    word.len() > 2 && word.ends_with("es") && sibilant(&word[..word.len() - 2])
}

fn sibilant(word: &str) -> bool {
    word.ends_with("ch")
        || word.ends_with("sh")
        || word.ends_with('x')
        || word.ends_with('z')
        || word.ends_with("ss")
        || (word.len() >= 2 && word.ends_with('s') && !word.ends_with("es") && vowel(nth_last(word, 2)))
}

fn nth_last(word: &str, n: usize) -> char {
    word.as_bytes()[word.len() - n] as char
}

fn vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn consonant(c: char) -> bool {
    c.is_ascii_alphabetic() && !vowel(c)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
