/*
 * Isabelle project
 *
 * Copyright 2023-2024 Maxim Menshikov
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the “Software”),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */
use crate::data_model::item::*;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(untagged)]
enum IdEntry {
    Pair((u16, u64)),
    Bare(u64),
}

impl IdEntry {
    fn into_pair(self) -> (u16, u64) {
        match self {
            IdEntry::Pair(p) => p,
            IdEntry::Bare(n) => (0u16, n),
        }
    }
}

pub fn deserialize_id_map<'de, D>(d: D) -> Result<HashMap<String, (u16, u64)>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: HashMap<String, IdEntry> = HashMap::deserialize(d)?;
    Ok(raw.into_iter().map(|(k, v)| (k, v.into_pair())).collect())
}

pub fn deserialize_strid_map<'de, D>(
    d: D,
) -> Result<HashMap<String, Vec<(u16, u64)>>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: HashMap<String, Vec<IdEntry>> = HashMap::deserialize(d)?;
    Ok(raw
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().map(IdEntry::into_pair).collect()))
        .collect())
}

pub fn unset_item_map() -> HashMap<u64, Item> {
    return HashMap::new();
}

pub fn unset_bool() -> bool {
    return false;
}

pub fn unset_str() -> String {
    return "".to_string();
}

pub fn unset_str_map() -> HashMap<String, String> {
    return HashMap::new();
}

pub fn unset_strid_map() -> HashMap<String, Vec<(u16, u64)>> {
    return HashMap::new();
}

pub fn unset_str_vec() -> Vec<String> {
    return Vec::new();
}

pub fn unset_strstr_map() -> HashMap<String, HashMap<String, String>> {
    return HashMap::new();
}

pub fn unset_bool_map() -> HashMap<String, bool> {
    return HashMap::new();
}

pub fn unset_u64_map() -> HashMap<String, u64> {
    return HashMap::new();
}

pub fn unset_id_map() -> HashMap<String, (u16, u64)> {
    return HashMap::new();
}

pub fn unset_u64_vec() -> Vec<u64> {
    return Vec::new();
}

pub fn unset_id() -> u64 {
    return u64::MAX;
}

pub fn unset_u64() -> u64 {
    return 0;
}

pub fn unset_u64_max() -> u64 {
    return u64::MAX;
}

pub fn unset_time() -> u64 {
    return 0;
}

pub fn unset_data_node() -> ItemDataNode {
    return ItemDataNode::new();
}

pub fn is_unset_id(v: &u64) -> bool {
    *v == u64::MAX
}

pub fn is_false(v: &bool) -> bool {
    !*v
}

pub fn is_empty_str(v: &String) -> bool {
    v.is_empty()
}

pub fn is_unset_action(a: &ItemDataNodeAction) -> bool {
    matches!(a, ItemDataNodeAction::None)
}

pub fn is_empty_data_node(n: &ItemDataNode) -> bool {
    n.value.is_empty()
        && n.value_type.is_empty()
        && !n.writable
        && matches!(n.action, ItemDataNodeAction::None)
        && n.subnodes.is_empty()
}
