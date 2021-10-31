use std::sync::Arc;

use crate::hash_types::worker::FetchedDataset;

use crate::datastore::prelude::*;
use crate::proto::SharedDataset;

pub struct Batch {
    pub(crate) memory: Memory,
    pub(crate) reload_state: Metaversion,
}

impl super::Batch for Batch {
    fn memory(&self) -> &Memory {
        &self.memory
    }

    fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    fn metaversion(&self) -> &Metaversion {
        &self.reload_state
    }

    fn metaversion_mut(&mut self) -> &mut Metaversion {
        &mut self.reload_state
    }

    fn maybe_reload(&mut self, _reload_state: Metaversion) -> Result<()> {
        // TODO ret these errors
        Error::from("Datasets are not updated");
        Ok(())
    }

    fn reload(&mut self) -> Result<()> {
        Error::from("Datasets are not updated");
        Ok(())
    }
}

impl Batch {
    pub fn new_from_dataset(dataset: &SharedDataset, experiment_run_id: &str) -> Result<Batch> {
        let dataset_name = dataset.shortname.clone();
        let dataset_size = dataset.contents.len();
        let mut memory = Memory::from_sizes(
            experiment_run_id,
            0,
            dataset_name.len(),
            0,
            dataset_size,
            false,
        )?;
        let reload_state = Metaversion::default();
        memory.set_header(&dataset_name)?;
        let buffer = memory.get_mut_data_buffer()?;
        buffer.copy_from_slice(dataset.contents.as_bytes());
        Ok(Batch {
            memory,
            reload_state,
        })
    }
}
