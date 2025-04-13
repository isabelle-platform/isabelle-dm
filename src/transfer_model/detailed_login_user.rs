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
use yew::prelude::*;

/// More detailed user information augmented with site data
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
#[repr(C)]
pub struct DetailedLoginUser {
    /// Username
    #[serde(default = "unset_str")]
    pub username: String,

    /// ID of the user
    #[serde(default = "unset_id")]
    pub id: u64,

    /// List of roles
    #[serde(default = "unset_str_vec")]
    pub role: Vec<String>,

    /// Site name
    #[serde(default = "unset_str")]
    pub site_name: String,

    /// Site logo
    #[serde(default = "unset_str")]
    pub site_logo: String,

    /// Licensing information
    #[serde(default = "unset_str")]
    pub licensed_to: String,

    /// Application-depended user parameters
    #[serde(default = "unset_str_map")]
    pub params: HashMap<String, String>,
}
