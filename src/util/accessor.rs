use std::collections::HashMap;

pub fn unset_str_map() -> HashMap<String, String> {
    return HashMap::new();
}

pub fn unset_bool_map() -> HashMap<String, bool> {
    return HashMap::new();
}

pub fn unset_id_map() -> HashMap<String, u64> {
    return HashMap::new();
}

pub fn unset_id() -> u64 {
    return u64::MAX;
}

pub fn unset_time() -> u64 {
    return 0;
}
