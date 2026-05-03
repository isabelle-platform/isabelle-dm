use isabelle_dm::data_model::item::{Item, ItemDataNode, ItemDataNodeAction};
use serde_json::{json, Value};

#[test]
fn merge_combines_collection_ids() {
    let mut a = Item::new();
    a.set_id("owner", (1u16, 10u64));
    a.set_strid("tags", &vec![(1u16, 1u64), (1u16, 2u64)]);

    let mut b = Item::new();
    b.set_id("owner", (2u16, 99u64)); // overwrite
    b.set_id("parent", (3u16, 7u64)); // new key
    b.set_strid("tags", &vec![(2u16, 5u64)]); // overwrite vec

    a.merge(&b);

    assert_eq!(a.safe_id("owner", (0, 0)), (2, 99));
    assert_eq!(a.safe_id("parent", (0, 0)), (3, 7));
    assert_eq!(a.safe_strid("tags", &vec![]), vec![(2, 5)]);
}

#[test]
fn set_id_overwrites_collection_component() {
    let mut itm = Item::new();
    itm.set_id("k", (1u16, 100u64));
    itm.set_id("k", (5u16, 100u64));
    assert_eq!(itm.safe_id("k", (0, 0)), (5, 100));

    itm.set_id("k", (5u16, 200u64));
    assert_eq!(itm.safe_id("k", (0, 0)), (5, 200));
}

#[test]
fn id_field_not_skipped_when_set() {
    let mut itm = Item::new();
    itm.id = 0; // valid, real id
    let v: Value = serde_json::from_str(&serde_json::to_string(&itm).unwrap()).unwrap();
    assert_eq!(v, json!({ "id": 0 }));
}

#[test]
fn partial_root_node_emits_only_filled_subfields() {
    let mut itm = Item::new();
    let mut node = ItemDataNode::new();
    node.value = "hello".to_string();
    node.action = ItemDataNodeAction::Edit;
    // value_type empty, writable false, subnodes empty — must all be skipped.
    itm.root_node = node;

    let v: Value = serde_json::from_str(&serde_json::to_string(&itm).unwrap()).unwrap();
    assert_eq!(
        v,
        json!({
            "root_node": { "value": "hello", "action": "Edit" }
        })
    );
}

#[test]
fn nested_subnodes_skip_their_own_empty_fields() {
    let mut itm = Item::new();
    let mut leaf = ItemDataNode::new();
    leaf.value = "v".to_string();

    let mut mid = ItemDataNode::new();
    mid.subnodes.insert("leaf".to_string(), leaf);

    itm.root_node.subnodes.insert("mid".to_string(), mid);

    let v: Value = serde_json::from_str(&serde_json::to_string(&itm).unwrap()).unwrap();
    assert_eq!(
        v,
        json!({
            "root_node": {
                "subnodes": {
                    "mid": {
                        "subnodes": {
                            "leaf": { "value": "v" }
                        }
                    }
                }
            }
        })
    );
}

#[test]
fn malformed_id_payload_is_rejected() {
    // String where number/pair is expected — must error, not silently accept.
    let bad = r#"{ "ids": { "k": "not-a-number" } }"#;
    assert!(serde_json::from_str::<Item>(bad).is_err());

    // Pair with wrong arity.
    let bad2 = r#"{ "ids": { "k": [1, 2, 3] } }"#;
    assert!(serde_json::from_str::<Item>(bad2).is_err());

    // strids element of wrong shape.
    let bad3 = r#"{ "strids": { "k": [ "x" ] } }"#;
    assert!(serde_json::from_str::<Item>(bad3).is_err());
}
