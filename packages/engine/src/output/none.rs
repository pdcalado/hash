use crate::proto::SimulationShortID;
use serde_json::Value;

use crate::config::PersistenceConfig;
use crate::error::Result;
use crate::output::OutputPersistenceResultRepr;
use crate::simulation::{
    packages::output::packages::OutputPackagesSimConfig, step_result::SimulationStepResult,
};

use super::{OutputPersistenceCreatorRepr, SimulationOutputPersistenceRepr};

pub struct NoOutputPersistence {}

impl NoOutputPersistence {
    pub fn new() -> NoOutputPersistence {
        NoOutputPersistence {}
    }
}

impl OutputPersistenceCreatorRepr for NoOutputPersistence {
    type SimulationOutputPersistence = NoSimulationOutputPersistence;

    fn new_simulation(
        &self,
        sim_id: SimulationShortID,
        persistence_config: &PersistenceConfig,
    ) -> Result<Self::SimulationOutputPersistence> {
        Ok(NoSimulationOutputPersistence {})
    }
}

struct NoSimulationOutputPersistence {}

#[async_trait::async_trait]
impl SimulationOutputPersistenceRepr for NoSimulationOutputPersistence {
    type OutputPersistenceResult = ();

    async fn add_step_output(&mut self, output: SimulationStepResult) -> Result<()> {
        Ok(())
    }

    async fn finalize(self) -> Result<Self::OutputPersistenceResult> {
        Ok(())
    }
}

impl OutputPersistenceResultRepr for () {
    fn as_value(self) -> Result<(&'static str, Value)> {
        Ok(("none", Value::Null))
    }
}
