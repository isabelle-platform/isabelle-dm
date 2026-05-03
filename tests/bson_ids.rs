use bson::{bson, doc, to_bson, Bson};
use isabelle_dm::data_model::item::Item;

#[test]
fn bson_round_trip_pair_ids() {
    let mut itm = Item::new();
    itm.id = 1;
    itm.set_id("ref", (7u16, 42u64));
    itm.set_strid("refs", &vec![(1u16, 10u64), (2u16, 20u64)]);

    let b = to_bson(&itm).expect("serialize to bson");
    let back: Item = bson::from_bson(b).expect("deserialize from bson");

    assert_eq!(back.safe_id("ref", (0, 0)), (7, 42));
    assert_eq!(back.safe_strid("refs", &vec![]), vec![(1, 10), (2, 20)]);
}

#[test]
fn bson_legacy_bare_number_ids_decode_with_zero_collection() {
    // Simulate a document stored before the (u16, u64) migration:
    // ids/strids contain bare numbers, not arrays-of-two.
    let raw = doc! {
        "id": 1i64,
        "strs": {},
        "strstrs": {},
        "strids": { "refs": [ 10i64, 20i64 ] },
        "bools": {},
        "u64s": {},
        "ids": { "ref": 42i64 },
        "root_node": {
            "value": "",
            "value_type": "",
            "writable": false,
            "action": "None",
            "subnodes": {},
        },
    };

    let itm: Item = bson::from_bson(Bson::Document(raw)).expect("deserialize legacy bson");

    assert_eq!(itm.safe_id("ref", (9, 9)), (0, 42));
    assert_eq!(itm.safe_strid("refs", &vec![]), vec![(0, 10), (0, 20)]);
}

#[test]
fn bson_mixed_pair_and_bare() {
    let raw = doc! {
        "id": 2i64,
        "strids": { "mix": [ 5i64, [ 3i32, 7i64 ] ] },
        "ids": { "old": 99i64, "new": [ 4i32, 8i64 ] },
        "root_node": {
            "value": "", "value_type": "", "writable": false,
            "action": "None", "subnodes": {},
        },
    };

    let itm: Item = bson::from_bson(Bson::Document(raw)).expect("deserialize mixed bson");

    assert_eq!(itm.safe_id("old", (0, 0)), (0, 99));
    assert_eq!(itm.safe_id("new", (0, 0)), (4, 8));
    assert_eq!(itm.safe_strid("mix", &vec![]), vec![(0, 5), (3, 7)]);
}

#[test]
fn bson_pair_serializes_as_array() {
    // Sanity-check that a tuple round-trips through BSON as a 2-element array,
    // i.e. MongoDB will store it as an array document.
    let pair: (u16, u64) = (3, 99);
    let b = to_bson(&pair).expect("tuple to bson");
    assert_eq!(b, bson!([3i32, 99i64]));
}
