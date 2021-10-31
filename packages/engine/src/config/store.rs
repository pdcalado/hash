use crate::hash_types::Properties;
use crate::proto::ExperimentRunBase;

use crate::{
    datastore::{
        prelude::*,
        schema::{
            context::ContextSchema,
            state::{AgentSchema, MessageSchema},
        },
    },
    simulation::packages::creator::PackageCreators,
};
use std::sync::Arc;

pub struct Config {
    pub agent_schema: Arc<AgentSchema>,
    pub message_schema: Arc<MessageSchema>,
    pub context_schema: Arc<ContextSchema>,
}

impl Config {
    pub fn new_sim(
        exp_config: &super::ExperimentConfig<ExperimentRunBase>,
        globals: &Properties,
        package_creators: &PackageCreators,
    ) -> Result<Config> {
        let agent_schema = Arc::new(package_creators.get_agent_schema(exp_config, globals)?);
        let message_schema = Arc::new(MessageSchema::new());
        let context_schema = Arc::new(package_creators.get_context_schema(exp_config, globals)?);

        Ok(Config {
            agent_schema,
            message_schema,
            context_schema,
        })
    }
}
