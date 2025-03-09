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
use crate::util::accessor::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
#[repr(C)]
pub struct ItemDataNode {
    #[serde(default = "unset_str")]
    pub value: String,
    #[serde(default = "unset_str")]
    pub value_type: String,
    #[serde(default = "unset_bool")]
    pub writable: bool,
    #[serde(default = "unset_bool")]
    pub edited: bool,
    pub subnodes: HashMap<String, ItemDataNode>,
}

unsafe impl Send for ItemDataNode {}

impl ItemDataNode {
    pub fn new() -> Self {
        Self {
            value: "".to_string(),
            value_type: "".to_string(),
            writable: false,
            edited: false,
            subnodes: HashMap::new(),
        }
    }

    pub fn get_subnode(&self, name: &str) -> Option<ItemDataNode> {
        if self.subnodes.contains_key(name) {
            return Some(self.subnodes[name].clone());
        }
        return None;
    }

    pub fn merge(&mut self, node: &ItemDataNode) {
        self.value = node.value.clone();
        self.value_type = node.value_type.clone();
        for subnode in &node.subnodes {
            let idx = subnode.0.clone();

            if !self.subnodes.contains_key(&idx) {
                let tmp = ItemDataNode::new();
                self.subnodes.insert(idx.clone(), tmp);
            }

            let new_node = self.subnodes.get_mut(&idx).unwrap();
            new_node.merge(&subnode.1);
        }
    }
}

/// Main isabelle object structure carrying hash maps
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
#[repr(C)]
pub struct Item {
    #[serde(default = "unset_id")]
    pub id: u64,

    #[serde(default = "unset_str_map")]
    pub strs: HashMap<String, String>,

    #[serde(default = "unset_strstr_map")]
    pub strstrs: HashMap<String, HashMap<String, String>>,

    #[serde(default = "unset_strid_map")]
    pub strids: HashMap<String, HashMap<u64, bool>>,

    #[serde(default = "unset_bool_map")]
    pub bools: HashMap<String, bool>,

    #[serde(default = "unset_u64_map")]
    pub u64s: HashMap<String, u64>,

    #[serde(default = "unset_u64_map")]
    pub ids: HashMap<String, u64>,

    #[serde(default = "unset_data_node")]
    pub root_node: ItemDataNode,
}

unsafe impl Send for Item {}

impl Item {
    pub fn new() -> Self {
        Self {
            id: u64::MAX,
            strs: HashMap::new(),
            strids: HashMap::new(),
            strstrs: HashMap::new(),
            bools: HashMap::new(),
            u64s: HashMap::new(),
            ids: HashMap::new(),
            root_node: ItemDataNode::new(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn safe_str(&self, name: &str, def: &str) -> String {
        if self.strs.contains_key(name) {
            self.strs[name].clone()
        } else {
            def.to_string()
        }
    }

    pub fn safe_str_with_empty(&self, name: &str, def: &str) -> String {
        if self.strs.contains_key(name) {
            if self.strs[name] == "" {
                def.to_string()
            } else {
                self.strs[name].clone()
            }
        } else {
            def.to_string()
        }
    }

    pub fn set_str(&mut self, name: &str, val: &str) {
        if self.strs.contains_key(name) {
            let v = self.strs.get_mut(name).unwrap();
            *v = val.to_string();
        } else {
            self.strs.insert(name.to_string(), val.to_string());
        }
    }

    pub fn safe_strstr(
        &self,
        name: &str,
        def: &HashMap<String, String>,
    ) -> HashMap<String, String> {
        if self.strstrs.contains_key(name) {
            self.strstrs[name].clone()
        } else {
            def.clone()
        }
    }

    pub fn safe_strid(&self, name: &str, def: &HashMap<u64, bool>) -> HashMap<u64, bool> {
        if self.strids.contains_key(name) {
            self.strids[name].clone()
        } else {
            def.clone()
        }
    }

    pub fn set_strid(&mut self, name: &str, val: &HashMap<u64, bool>) {
        if self.strids.contains_key(name) {
            let v = self.strids.get_mut(name).unwrap();
            *v = val.clone();
        } else {
            self.strids.insert(name.to_string(), val.clone());
        }
    }

    pub fn set_strstr(&mut self, name: &str, val: &HashMap<String, String>) {
        if self.strstrs.contains_key(name) {
            let v = self.strstrs.get_mut(name).unwrap();
            *v = val.clone();
        } else {
            self.strstrs.insert(name.to_string(), val.clone());
        }
    }

    pub fn safe_bool(&self, name: &str, def: bool) -> bool {
        if self.bools.contains_key(name) {
            self.bools[name].clone()
        } else {
            def
        }
    }

    pub fn set_bool(&mut self, name: &str, val: bool) {
        let mut full_name = name;
        if name.starts_with("negated_") {
            full_name = &name[8..];
        }
        if self.bools.contains_key(full_name) {
            let v = self.bools.get_mut(full_name).unwrap();
            *v = val;
        } else {
            self.bools.insert(full_name.to_string(), val);
        }
    }

    pub fn safe_u64(&self, name: &str, def: u64) -> u64 {
        if self.u64s.contains_key(name) {
            self.u64s[name].clone()
        } else {
            def
        }
    }

    pub fn set_u64(&mut self, name: &str, val: u64) {
        if self.u64s.contains_key(name) {
            let v = self.u64s.get_mut(name).unwrap();
            *v = val;
        } else {
            self.u64s.insert(name.to_string(), val);
        }
    }

    pub fn safe_id(&self, name: &str, def: u64) -> u64 {
        if self.ids.contains_key(name) {
            self.ids[name].clone()
        } else {
            def
        }
    }

    pub fn set_id(&mut self, name: &str, val: u64) {
        if self.ids.contains_key(name) {
            let v = self.ids.get_mut(name).unwrap();
            *v = val;
        } else {
            self.ids.insert(name.to_string(), val);
        }
    }

    pub fn merge(&mut self, itm: &Item) {
        for obj in &itm.strs {
            self.set_str(obj.0, obj.1);
        }

        for obj in &itm.strstrs {
            self.set_strstr(obj.0, obj.1);
        }

        for obj in &itm.bools {
            self.set_bool(obj.0, *obj.1);
        }

        for obj in &itm.u64s {
            self.set_u64(obj.0, *obj.1);
        }

        for obj in &itm.ids {
            self.set_id(obj.0, *obj.1);
        }

        for obj in &itm.strids {
            self.set_strid(obj.0, obj.1);
        }

        self.root_node.merge(&itm.root_node);
    }

    pub fn normalize_negated(&mut self) {
        let bools = self.bools.clone();
        for obj in &bools {
            if obj.0.starts_with("negated_") && bools.contains_key(&obj.0[8..]) {
                self.bools.remove(obj.0);
            }
        }
    }

    pub fn get_node(&self, path: &str) -> Option<ItemDataNode> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut cur_node = &self.root_node;

        for part in parts {
            if !cur_node.subnodes.contains_key(part) {
                return None;
            }

            cur_node = &cur_node.subnodes[part];
        }

        return Some(cur_node.clone());
    }

    pub fn set_node(&mut self, path: &str, data_node: &ItemDataNode) {
        let parts: Vec<&str> = path.split('.').collect();
        let mut cur_node = &mut self.root_node;

        for part in parts {
            if !cur_node.subnodes.contains_key(part) {
                cur_node
                    .subnodes
                    .insert(part.to_string(), ItemDataNode::new());
            }

            cur_node = cur_node.subnodes.get_mut(part).unwrap();
        }

        cur_node.value = data_node.value.clone();
        cur_node.value_type = data_node.value_type.clone();
        cur_node.writable = data_node.writable.clone();
        cur_node.edited = data_node.edited.clone();
        cur_node.subnodes = data_node.subnodes.clone();
    }
}
