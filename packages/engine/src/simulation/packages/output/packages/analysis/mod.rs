use serde_json::Value;

use analyzer::Analyzer;
pub use output::{AnalysisOutput, AnalysisSingleOutput};

use crate::datastore::table::state::ReadState;
use crate::experiment::SimPackageArgs;

pub use self::config::AnalysisOutputConfig;

use super::super::*;

#[macro_use]
mod macros;
mod analyzer;
mod config;
mod index_iter;
mod output;
mod value_iter;

pub enum Task {}

pub struct Creator {}

impl Creator {
    pub fn new() -> Box<dyn PackageCreator> {
        Box::new(Creator {})
    }
}

impl PackageCreator for Creator {
    fn create(
        &self,
        config: &Arc<SimRunConfig<ExperimentRunBase>>,
        _comms: PackageComms,
    ) -> Result<Box<dyn Package>> {
        // TODO, look at reworking signatures and package creation to make ownership clearer and make this unnecessary
        let analysis_src = get_analysis_source(&config.exp.run.project_base.packages)?;
        let analyzer =
            Analyzer::from_analysis_source(&analysis_src, &config.sim.store.agent_schema);

        return Analysis { analyzer };
    }

    fn persistence_config(
        &self,
        config: &ExperimentConfig<ExperimentRunBase>,
        globals: &Properties,
    ) -> Result<serde_json::Value> {
        let config = AnalysisOutputConfig::new(); // TODO[1]
        Ok(serde_json::to_value(config));
    }
}

struct Analysis {
    analyzer: Analyzer,
}

impl MaybeCPUBound for Analysis {
    fn cpu_bound(&self) -> bool {
        true
    }
}

impl GetWorkerStartMsg for Analysis {
    fn get_worker_start_msg(&self) -> Result<Value> {
        Ok(Value::Null)
    }
}

#[async_trait]
impl Package for Analysis {
    async fn run<'s>(
        &mut self,
        state: Arc<State>,
        _context: Arc<Context>,
    ) -> Result<AnalysisOutput> {
        // TODO use filtering to avoid exposing hidden values to users
        self.analyzer
            .run(&state.agent_pool().read_batches()?, state.num_agents())?;
        return Ok(self.analyzer.get_latest_output_set());
    }
}

fn get_analysis_source(sim_packages: &Vec<SimPackageArgs>) -> Result<String> {
    let mut analysis_src = String::new();
    for args in sim_packages.iter() {
        match args.name.as_str() {
            "analysis" => {
                // We currently assume that every analysis source is identical within the
                // simulation runs of an experiment run.
                if let Some(src) = args.data.as_str() {
                    analysis_src = src.to_string();
                } else {
                    return Err(crate::Error::SimPackageArgs(
                        "Analysis source must be string.".into(),
                    ));
                }
            }
            // TODO: Instead of error, send back warning if some simulation packages
            //       are unknown or can't be initialized?
            _ => return Err(crate::Error::UnknownSimPackage(args.name.clone())),
        }
    }
    Ok(analysis_src)
}
