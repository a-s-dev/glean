/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! THis module defines all the information needed to match a user with an experiment
//! this module should also include a `match` function of some sort that does the matching
//! tt has two main types, the `matcher` retrieved from the server, and the `AppContext`
//! from the client
use super::msg_types;
use serde_derive::*;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Matcher {
    pub app_id: Option<String>,
    pub app_display_version: Option<String>,
    pub app_min_version: Option<String>, // Do what AC does and have a VersionOptionString instead?
    pub app_max_version: Option<String>, //Dito
    pub locale_language: Option<String>,
    pub locale_country: Option<String>,
    pub device_manufacturer: Option<String>,
    pub device_model: Option<String>,
    pub regions: Vec<String>,
    pub debug_tags: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AppContext {
    pub app_id: Option<String>,
    pub app_version: Option<String>,
    pub locale_language: Option<String>,
    pub locale_country: Option<String>,
    pub device_manufacturer: Option<String>,
    pub device_model: Option<String>,
    pub region: Option<String>,
    pub debug_tag: Option<String>,
}

impl From<msg_types::AppContext> for AppContext {
    fn from(proto_ctx: msg_types::AppContext) -> Self {
        Self {
            app_id: proto_ctx.app_id,
            app_version: proto_ctx.app_version,
            locale_language: proto_ctx.locale_language,
            locale_country: proto_ctx.locale_country,
            device_manufacturer: proto_ctx.device_manufacturer,
            device_model: proto_ctx.device_model,
            region: proto_ctx.region,
            debug_tag: proto_ctx.debug_tag,
        }
    }
}
