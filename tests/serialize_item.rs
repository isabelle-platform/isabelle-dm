use bson::{bson, to_bson, Bson};
use isabelle_dm::data_model::item::Item;
use serde_json::{json, Value};

fn make_item() -> Item {
    let mut itm = Item::new();
    itm.id = 7;
    itm.set_id("owner", (3u16, 42u64));
    itm.set_id("parent", (0u16, 100u64));
    itm.set_strid("tags", &vec![(1u16, 10u64), (2u16, 20u64)]);
    itm.set_strid("empty", &vec![]);
    itm
}

#[test]
fn json_serializes_ids_as_pair_arrays() {
    let itm = make_item();
    let v: Value = serde_json::from_str(&serde_json::to_string(&itm).unwrap()).unwrap();

    assert_eq!(v["id"], json!(7));
    assert_eq!(v["ids"]["owner"], json!([3, 42]));
    assert_eq!(v["ids"]["parent"], json!([0, 100]));
}

#[test]
fn json_serializes_strids_as_arrays_of_pair_arrays() {
    let itm = make_item();
    let v: Value = serde_json::from_str(&serde_json::to_string(&itm).unwrap()).unwrap();

    assert_eq!(v["strids"]["tags"], json!([[1, 10], [2, 20]]));
    assert_eq!(v["strids"]["empty"], json!([]));
}

#[test]
fn json_zero_collection_still_emits_pair() {
    // After migration, even a (0, n) value must be written as a pair, not a bare number.
    let mut itm = Item::new();
    itm.set_id("owner", (0u16, 50u64));
    let v: Value = serde_json::from_str(&serde_json::to_string(&itm).unwrap()).unwrap();
    assert_eq!(v["ids"]["owner"], json!([0, 50]));
    assert!(!v["ids"]["owner"].is_number());
}

#[test]
fn json_full_round_trip_preserves_pairs() {
    let itm = make_item();
    let s = serde_json::to_string(&itm).unwrap();
    let back: Item = serde_json::from_str(&s).unwrap();

    assert_eq!(back.safe_id("owner", (9, 9)), (3, 42));
    assert_eq!(back.safe_id("parent", (9, 9)), (0, 100));
    assert_eq!(back.safe_strid("tags", &vec![]), vec![(1, 10), (2, 20)]);
    assert_eq!(
        back.safe_strid("empty", &vec![(9, 9)]),
        Vec::<(u16, u64)>::new()
    );
}

#[test]
fn bson_serializes_ids_as_pair_arrays() {
    let itm = make_item();
    let b = to_bson(&itm).expect("to bson");
    let doc = b.as_document().expect("bson is a document");

    let ids = doc.get_document("ids").expect("ids");
    assert_eq!(ids.get("owner").unwrap(), &bson!([3i32, 42i64]));
    assert_eq!(ids.get("parent").unwrap(), &bson!([0i32, 100i64]));
}

#[test]
fn bson_serializes_strids_as_arrays_of_pair_arrays() {
    let itm = make_item();
    let b = to_bson(&itm).expect("to bson");
    let doc = b.as_document().expect("bson is a document");

    let strids = doc.get_document("strids").expect("strids");
    assert_eq!(
        strids.get("tags").unwrap(),
        &bson!([[1i32, 10i64], [2i32, 20i64]])
    );
    assert_eq!(strids.get("empty").unwrap(), &Bson::Array(vec![]));
}

#[test]
fn bson_full_round_trip_preserves_pairs() {
    let itm = make_item();
    let b = to_bson(&itm).expect("to bson");
    let back: Item = bson::from_bson(b).expect("from bson");

    assert_eq!(back.id, 7);
    assert_eq!(back.safe_id("owner", (9, 9)), (3, 42));
    assert_eq!(back.safe_id("parent", (9, 9)), (0, 100));
    assert_eq!(back.safe_strid("tags", &vec![]), vec![(1, 10), (2, 20)]);
}

#[test]
fn empty_item_serializes_to_empty_object() {
    // All fields are skipped when empty/unset, including id == u64::MAX.
    let itm = Item::new();
    let v: Value = serde_json::from_str(&serde_json::to_string(&itm).unwrap()).unwrap();
    assert_eq!(v, json!({}));
}

#[test]
fn empty_item_now_serializes_to_bson() {
    // Previously this failed with UnsignedIntegerExceededRange because
    // id = u64::MAX. With skip_serializing_if it must succeed.
    let itm = Item::new();
    let b = bson::to_bson(&itm).expect("empty item must serialize to bson");
    let doc = b.as_document().unwrap();
    assert!(doc.is_empty(), "expected empty bson doc, got {:?}", doc);
}

#[test]
fn partially_filled_item_only_emits_filled_fields() {
    let mut itm = Item::new();
    itm.set_str("name", "alpha");
    let v: Value = serde_json::from_str(&serde_json::to_string(&itm).unwrap()).unwrap();
    let obj = v.as_object().unwrap();
    assert_eq!(obj.len(), 1);
    assert_eq!(obj.get("strs"), Some(&json!({ "name": "alpha" })));
}
