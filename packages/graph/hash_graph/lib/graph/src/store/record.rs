use std::fmt::Debug;

use crate::{
    store::query::{Filter, QueryPath},
    subgraph::SubgraphIndex,
};

/// A record stored in the [`store`].
///
/// [`store`]: crate::store
pub trait Record: Sized + Send {
    type EditionId: Debug + SubgraphIndex<Self> + Send + Sync;
    type QueryPath<'p>: QueryPath + Send + Sync;

    fn edition_id(&self) -> &Self::EditionId;

    fn create_filter_for_edition_id(edition_id: &Self::EditionId) -> Filter<Self>;
}
