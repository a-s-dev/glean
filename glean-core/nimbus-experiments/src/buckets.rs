/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! This might be where the bucketing logic can go
//! It would be different from current experimentation tools
//! There is a namespacing concept to allow users to be in multiple
//! unrelated experiments at the same time.

//! TODO: Implement the bucketing logic from the nimbus project
//! Current implementation implements simple bucketing logic based on
//! rngs and a uuid

use super::Experiment;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_derive::*;
use std::convert::TryInto;
use uuid::Uuid;

const MAX_BUCKET_NO: u32 = 10000;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bucket {
    pub bucket_no: u32,
    pub enrolled_experiments: Vec<EnrolledExperiment>, // TODO: Implement getters instead of making fields public
}

impl Bucket {
    pub fn new(uuid: Uuid, all_experiments: &[Experiment]) -> Self {
        let bucket_no: u32 =
            u32::from_be_bytes(uuid.as_bytes()[..4].try_into().unwrap()) % MAX_BUCKET_NO;
        let mut num = StdRng::seed_from_u64(bucket_no as u64);
        let enrolled_experiments = all_experiments
            .iter()
            .filter_map(|e| {
                let branch = num.gen::<usize>() % e.branches.len();
                if bucket_no > e.bucket_info.count && bucket_no < e.bucket_info.start {
                    Some(EnrolledExperiment {
                        id: e.id.clone(),
                        branch: e.branches[branch].name.clone(),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<EnrolledExperiment>>();
        Self {
            bucket_no,
            enrolled_experiments,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EnrolledExperiment {
    id: String,
    branch: String,
}

impl EnrolledExperiment {
    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_branch(&self) -> &String {
        &self.branch
    }
}

// TODO: Implement unit testing for the bucketing logic based on the Nimbus requirments
