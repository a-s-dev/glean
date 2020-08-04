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
use chrono::{DateTime, Utc};
use error::Result;
pub use ffi::{experiements_destroy, experiments_get_branch, experiments_new};
use http_client::{Client, SettingsClient};
pub use matcher::AppContext;
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

pub struct Config {
    server_url: Option<String>,
    uuid: Option<Uuid>,
    collection_name: Option<String>,
    bucket_name: Option<String>,
}

impl Experiments {
    /// A new experiments struct is created, this is where some preprocessing happens
    /// It should look for persisted state first and setup some type
    /// Of interval retrieval from the server for any experiment updates (not implemented)
    pub fn new<P: AsRef<Path>>(app_ctx: AppContext, path: P, config: Option<Config>) -> Self {
        let database = Database::new(path).unwrap();
        let persisted_data = database.get::<Self>("persisted").unwrap();
        if let Some(inner) = persisted_data {
            log::info!("Retrieving data from persisted state...");
            return inner;
        }

        let (base_url, collection_name, bucket_name, uuid) = match config {
            Some(config) => (
                config.server_url.unwrap_or(BASE_URL.to_string()),
                config
                    .collection_name
                    .unwrap_or(COLLECTION_NAME.to_string()),
                config.bucket_name.unwrap_or(BUCKET_NAME.to_string()),
                config.uuid.unwrap_or_else(|| uuid::Uuid::new_v4()),
            ),
            None => (
                BASE_URL.to_string(),
                COLLECTION_NAME.to_string(),
                BUCKET_NAME.to_string(),
                uuid::Uuid::new_v4(),
            ),
        };

        // TODO: Implement a type of interval schedule to check with the server for any
        // updates
        let http_client = Client::new(Url::parse(&base_url).unwrap(), collection_name, bucket_name);
        let resp = http_client.get_experiments().unwrap();
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Experiment {
    pub id: String,
    pub filter_expression: String,
    pub targeting: Option<String>,
    pub enabled: bool,
    pub arguments: ExperimentArguments,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExperimentArguments {
    pub slug: String,
    pub user_facing_name: String,
    pub user_facing_description: String,
    pub active: bool,
    pub is_enrollment_paused: bool,
    pub bucket_config: BucketConfig,
    pub features: Vec<String>,
    pub branches: Vec<Branch>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub proposed_duration: u64,
    pub proposed_enrollment: u64,
    pub reference_branch: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Branch {
    pub slug: String,
    pub ratio: u32,
    pub group: Option<Vec<Group>>,
    pub value: BranchValue,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Group {
    Cfr,
    AboutWelcome,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BranchValue {}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BucketConfig {
    pub randomization_unit: RandomizationUnit,
    pub namespace: String,
    pub start: u32,
    pub count: u32,
    pub total: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RandomizationUnit {
    ClientId,
    NormandyId,
    UserId,
}

// TODO: Implement unit tests
// A We could use mockiato to mock the trait functions
// of the http client, and test the main functionality here
