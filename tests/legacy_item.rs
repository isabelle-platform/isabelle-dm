use isabelle_dm::data_model::item::Item;

#[test]
fn json_legacy_item_full_shape() {
    // Pre-migration shape: ids/strids hold bare numbers.
    let raw = r#"{
        "id": 100,
        "strs": { "name": "alpha" },
        "strstrs": { "meta": { "k": "v" } },
        "strids": { "refs": [10, 20, 30] },
        "bools": { "active": true },
        "u64s": { "size": 5 },
        "ids": { "owner": 42, "parent": 7 },
        "root_node": {
            "value": "",
            "value_type": "",
            "writable": false,
            "action": "None",
            "subnodes": {}
        }
    }"#;

    let itm: Item = serde_json::from_str(raw).expect("parse legacy json");

    assert_eq!(itm.id, 100);
    assert_eq!(itm.safe_str("name", ""), "alpha");
    assert_eq!(itm.safe_bool("active", false), true);
    assert_eq!(itm.safe_u64("size", 0), 5);
    assert_eq!(itm.safe_id("owner", (9, 9)), (0, 42));
    assert_eq!(itm.safe_id("parent", (9, 9)), (0, 7));
    assert_eq!(
        itm.safe_strid("refs", &vec![]),
        vec![(0, 10), (0, 20), (0, 30)]
    );
}

#[test]
fn json_legacy_item_minimal_missing_id_fields() {
    // Even older payload that omits ids/strids entirely.
    let raw = r#"{ "id": 1 }"#;

    let itm: Item = serde_json::from_str(raw).expect("parse minimal legacy json");

    assert_eq!(itm.id, 1);
    assert_eq!(itm.ids.len(), 0);
    assert_eq!(itm.strids.len(), 0);
    // Defaults flow through safe accessors.
    assert_eq!(itm.safe_id("anything", (3, 4)), (3, 4));
    assert_eq!(itm.safe_strid("anything", &vec![(1, 2)]), vec![(1, 2)]);
}

#[test]
fn json_legacy_item_empty_collections() {
    let raw = r#"{
        "id": 2,
        "strids": { "empty": [] },
        "ids": {}
    }"#;

    let itm: Item = serde_json::from_str(raw).expect("parse legacy json");

    assert_eq!(
        itm.safe_strid("empty", &vec![(9, 9)]),
        Vec::<(u16, u64)>::new()
    );
    assert_eq!(itm.ids.len(), 0);
}

#[test]
fn json_mixed_legacy_and_new_pairs() {
    // Document written halfway through migration: some entries already pairs,
    // others still bare numbers.
    let raw = r#"{
        "id": 3,
        "ids": { "old": 99, "new": [4, 8] },
        "strids": { "mix": [5, [3, 7], 11] }
    }"#;

    let itm: Item = serde_json::from_str(raw).expect("parse mixed json");

    assert_eq!(itm.safe_id("old", (0, 0)), (0, 99));
    assert_eq!(itm.safe_id("new", (0, 0)), (4, 8));
    assert_eq!(
        itm.safe_strid("mix", &vec![]),
        vec![(0, 5), (3, 7), (0, 11)]
    );
}

#[test]
fn json_legacy_round_trip_after_upgrade() {
    // Read a legacy doc, then re-serialize and re-read: collection id 0 must persist.
    let legacy = r#"{ "id": 4, "ids": { "owner": 50 }, "strids": { "tags": [1, 2] } }"#;

    let itm: Item = serde_json::from_str(legacy).expect("parse legacy");
    let upgraded = serde_json::to_string(&itm).expect("serialize upgraded");
    let again: Item = serde_json::from_str(&upgraded).expect("parse upgraded");

    assert_eq!(again.safe_id("owner", (9, 9)), (0, 50));
    assert_eq!(again.safe_strid("tags", &vec![]), vec![(0, 1), (0, 2)]);
}
