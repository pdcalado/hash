use std::{
    collections::{
        hash_map::{RandomState, RawEntryMut},
        HashSet,
    },
    fmt::Debug,
    hash::Hash,
};

use edges::Edges;
use error_stack::Result;

use crate::{
    shared::identifier::GraphElementEditionId,
    store::{crud::Read, QueryError, Record},
    subgraph::{edges::GraphResolveDepths, vertices::Vertices},
};

pub mod edges;
pub mod query;
pub mod vertices;

#[derive(Debug)]
pub struct Subgraph {
    pub roots: HashSet<GraphElementEditionId>,
    pub vertices: Vertices,
    pub edges: Edges,
    pub depths: GraphResolveDepths,
}

impl Subgraph {
    #[must_use]
    pub fn new(depths: GraphResolveDepths) -> Self {
        Self {
            roots: HashSet::new(),
            vertices: Vertices::default(),
            edges: Edges::default(),
            depths,
        }
    }

    fn entry<R: Record>(
        &mut self,
        edition_id: &impl SubgraphIndex<R>,
    ) -> RawEntryMut<R::EditionId, R, RandomState> {
        edition_id.subgraph_entry(self)
    }

    pub fn insert<R: Record>(&mut self, record: R) -> Option<R> {
        match self.entry(record.edition_id()) {
            RawEntryMut::Occupied(mut entry) => Some(entry.insert(record)),
            RawEntryMut::Vacant(entry) => {
                entry.insert(record.edition_id().clone(), record);
                None
            }
        }
    }

    pub fn insert_as_root<R: Record>(&mut self, record: R) -> Option<R> {
        let edition_id = record.edition_id().clone();
        let record = match self.entry(&edition_id) {
            RawEntryMut::Occupied(mut entry) => Some(entry.insert(record)),
            RawEntryMut::Vacant(entry) => {
                entry.insert(edition_id.clone(), record);
                None
            }
        };
        self.roots.insert(edition_id.into());
        record
    }

    /// Looks up a single [`Record`] in the subgraph or reads it from the [`Store`] and inserts it
    /// if it is not yet in the subgraph.
    ///
    /// # Errors
    ///
    /// - Returns an error if the [`Record`] could not be read from the [`Store`].
    ///
    /// [`Store`]: crate::store::Store
    pub async fn get_or_read<'r, R: Record + Sync + 'r>(
        &'r mut self,
        store: &impl Read<R>,
        edition_id: &R::EditionId,
    ) -> Result<&'r R, QueryError> {
        Ok(match self.entry(edition_id) {
            RawEntryMut::Occupied(entry) => entry.into_mut(),
            RawEntryMut::Vacant(entry) => {
                entry
                    .insert(
                        edition_id.clone(),
                        store
                            .read_one(&R::create_filter_for_edition_id(edition_id))
                            .await?,
                    )
                    .1
            }
        })
    }
}

pub trait SubgraphIndex<R: Record>: Clone + Eq + Hash + Into<GraphElementEditionId> {
    fn subgraph_entry<'r>(
        &self,
        subgraph: &'r mut Subgraph,
    ) -> RawEntryMut<'r, R::EditionId, R, RandomState>;
}
