use std::collections::HashMap;

use crate::{output::local::config::LocalPersistenceConfig, Environment};

use serde::{Deserialize, Serialize};

use super::{Error, Result};

use crate::proto::ExperimentRunRepr;

pub const OUTPUT_PERSISTENCE_KEY: &'static str = "output_persistence";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OutputPersistenceConfig {
    Local(LocalPersistenceConfig),
    None,
}

pub fn output_persistence<E: ExperimentRunRepr>(
    env: &Environment<E>,
) -> Result<OutputPersistenceConfig> {
    get_dynamic(env, OUTPUT_PERSISTENCE_KEY)
}

pub fn get_dynamic<'de, K: Deserialize<'de>, E: ExperimentRunRepr>(
    env: &Environment<E>,
    key: &str,
) -> Result<K> {
    env.dyn_payloads
        .get(key)
        .map(|value| serde_json::from_value(value.clone()))
        .ok_or_else(|| Error::MissingConfiguration(key.to_string()))?
        .into()
}
