use crate::defaults;
use crate::registry_state::registry;

pub fn reset() {
    let mut guard = registry();
    *guard = None;
    defaults::clear();
}
