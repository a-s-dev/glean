/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Experiments library that hopes to be cross-plateform.
//! Still a work in progress, but good enough for people to poke around

mod buckets;
pub mod error;
pub mod ffi;
mod http_client;
mod matcher;
mod persistence;

use buckets::{Bucket, EnrolledExperiment};
use error::Result;
pub use ffi::{experiements_destroy, experiments_get_branch, experiments_new};
use http_client::{Client, SettingsClient};
pub use matcher::AppContext;
use matcher::Matcher;
use persistence::Database;
use serde_derive::*;
use std::path::Path;
use url::Url;
use uuid::Uuid;
pub use viaduct; // We expose this for the client to set

const BASE_URL: &str = "https://kinto.dev.mozaws.net/v1/";
const COLLECTION_NAME: &str = "messaging-collection";
const BUCKET_NAME: &str = "main";

// We'll probably end up doing what is done in A-S with regards to
// protobufs if we take this route...
// But for now, using the build.rs seems convenient
// ref: https://github.com/mozilla/application-services/tree/main/tools/protobuf-gen
pub mod msg_types {
    include!(concat!(
        env!("OUT_DIR"),
        "/mozilla.telemetry.glean.protobuf.rs"
    ));
}

/// Experiments is the main struct representing the experiements state
/// It should hold all the information needed to communcate a specific user's
/// Experiementation status (note: This should have some type of uuid)
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Experiments {
    // Uuid not used yet, but we'll be using it later
    #[allow(unused)]
    uuid: Uuid,
    #[allow(unused)]
    app_ctx: AppContext,
    experiments: Vec<Experiment>,
    bucket: Bucket,
}

impl Experiments {
    /// A new experiments struct is created, this is where some preprocessing happens
    /// It should look for persisted state first and setup some type
    /// Of interval retrieval from the server for any experiment updates (not implemented)
    /// TODO: Add an optional `ServerConfig` that allows clients to set servier settings
    pub fn new<P: AsRef<Path>>(app_ctx: AppContext, path: P) -> Self {
        let database = Database::new(path).unwrap();
        let persisted_data = database.get::<Self>("persisted").unwrap();
        if let Some(inner) = persisted_data {
            log::info!("Retrieving data from persisted state...");
            return inner;
        }
        // TODO: Implement a type of interval schedule to check with the server for any
        // updates
        let http_client = Client::new(
            Url::parse(BASE_URL).unwrap(),
            COLLECTION_NAME.to_string(),
            BUCKET_NAME.to_string(),
        );
        let resp = http_client.get_experiments().unwrap();
        let uuid = uuid::Uuid::new_v4();
        let bucket = Bucket::new(uuid, &resp);
        let experiments = Self {
            app_ctx,
            uuid,
            experiments: resp,
            bucket,
        };

        database
            .put(
                "persisted", // TODO: Think about a more appropriate key situation for persistance
                &experiments,
            )
            .unwrap();
        experiments
    }

    /// Retrieves the branch the user is in, in the experiment. Errors if the user is not enrolled (This should be an option, but for ffi + test it errors)
    pub fn get_experiment_branch(&self, exp_name: &str) -> Result<String> {
        self.bucket
            .enrolled_experiments
            .iter()
            .find(|e| e.get_id() == exp_name)
            .map(|e| e.get_branch().clone())
            .ok_or_else(|| anyhow::format_err!("No branch").into()) // Should be returning an option! But for now...
    }

    pub fn get_enrolled_experiments(&self) -> &Vec<EnrolledExperiment> {
        &self.bucket.enrolled_experiments
    }

    pub fn get_experiments(&self) -> &Vec<Experiment> {
        &self.experiments
    }

    pub fn get_bucket(&self) -> u32 {
        self.bucket.bucket_no
    }
}

// ============ Below are a bunch of types that gets serialized/deserialized and stored in our `Experiments` struct ============
// ============ They currently follow the old schema, and need to be updated to match the new Nimbus schema         ============

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Experiment {
    pub id: String,
    pub description: String,
    pub last_modified: u64,
    pub schema_modified: Option<u64>,
    #[serde(rename = "buckets")]
    pub bucket_info: BucketInfo,
    pub branches: Vec<Branch>,
    #[serde(rename = "match")]
    pub matcher: Matcher,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BucketInfo {
    pub count: u32,
    pub start: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Branch {
    pub name: String,
    ratio: u32,
}

// TODO: Implement unit tests
// A We could use mockiato to mock the trait functions
// of the http client, and test the main functionality here
