use crate::registry_state::registry;

pub fn registered() -> Vec<String> {
    let guard = registry();
    match guard.as_ref() {
        Some(map) => {
            let mut names: Vec<String> = map.keys().cloned().collect();
            names.sort();
            names
        }
        None => Vec::new(),
    }
}
