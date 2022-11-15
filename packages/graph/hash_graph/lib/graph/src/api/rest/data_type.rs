//! Web routes for CRU operations on Data Types.

use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router,
};
use error_stack::IntoReport;
use futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use type_system::{uri::VersionedUri, DataType};
use utoipa::{OpenApi, ToSchema};

use super::api_resource::RoutedResource;
use crate::{
    api::rest::{read_from_store, report_to_status_code},
    ontology::{
        domain_validator::{DomainValidator, ValidateOntologyType},
        patch_id_and_parse, PersistedDataType, PersistedOntologyIdentifier,
        PersistedOntologyMetadata,
    },
    provenance::{CreatedById, OwnedById, UpdatedById},
    shared::identifier::GraphElementIdentifier,
    store::{query::Filter, BaseUriAlreadyExists, BaseUriDoesNotExist, DataTypeStore, StorePool},
    subgraph::{
        DataTypeStructuralQuery, EdgeKind, Edges, GraphResolveDepths, OutwardEdge, StructuralQuery,
        Subgraph, Vertex,
    },
};

#[derive(OpenApi)]
#[openapi(
    paths(
        create_data_type,
        get_data_types_by_query,
        get_data_type,
        get_latest_data_types,
        update_data_type
    ),
    components(
        schemas(
            CreateDataTypeRequest,
            UpdateDataTypeRequest,
            OwnedById,
            CreatedById,
            UpdatedById,
            PersistedOntologyIdentifier,
            PersistedOntologyMetadata,
            PersistedDataType,
            DataTypeStructuralQuery,
            GraphElementIdentifier,
            Vertex,
            EdgeKind,
            OutwardEdge,
            GraphResolveDepths,
            Subgraph,
            Edges
        )
    ),
    tags(
        (name = "DataType", description = "Data Type management API")
    )
)]
pub struct DataTypeResource;

impl RoutedResource for DataTypeResource {
    /// Create routes for interacting with data types.
    fn routes<P: StorePool + Send + 'static>() -> Router {
        // TODO: The URL format here is preliminary and will have to change.
        Router::new().nest(
            "/data-types",
            Router::new()
                .route(
                    "/",
                    post(create_data_type::<P>)
                        .get(get_latest_data_types::<P>)
                        .put(update_data_type::<P>),
                )
                .route("/query", post(get_data_types_by_query::<P>))
                .route("/:version_id", get(get_data_type::<P>)),
        )
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct CreateDataTypeRequest {
    #[schema(value_type = VAR_DATA_TYPE)]
    schema: serde_json::Value,
    owned_by_id: OwnedById,
    actor_id: CreatedById,
}

#[utoipa::path(
    post,
    path = "/data-types",
    request_body = CreateDataTypeRequest,
    tag = "DataType",
    responses(
        (status = 201, content_type = "application/json", description = "The metadata of the created data type", body = PersistedOntologyMetadata),
        (status = 422, content_type = "text/plain", description = "Provided request body is invalid"),

        (status = 409, description = "Unable to create data type in the store as the base data type URI already exists"),
        (status = 500, description = "Store error occurred"),
    ),
    request_body = CreateDataTypeRequest,
)]
async fn create_data_type<P: StorePool + Send>(
    body: Json<CreateDataTypeRequest>,
    pool: Extension<Arc<P>>,
    domain_validator: Extension<DomainValidator>,
) -> Result<Json<PersistedOntologyMetadata>, StatusCode> {
    let Json(CreateDataTypeRequest {
        schema,
        owned_by_id,
        actor_id,
    }) = body;

    let data_type: DataType = schema.try_into().into_report().map_err(|report| {
        tracing::error!(error=?report, "Couldn't convert schema to Data Type");
        StatusCode::UNPROCESSABLE_ENTITY
        // TODO - We should probably return more information to the client
        //  https://app.asana.com/0/1201095311341924/1202574350052904/f
    })?;

    domain_validator.validate(&data_type).map_err(|report| {
        tracing::error!(error=?report, id=data_type.id().to_string(), "Data Type ID failed to validate");
        StatusCode::UNPROCESSABLE_ENTITY
    })?;

    let mut store = pool.acquire().await.map_err(|report| {
        tracing::error!(error=?report, "Could not acquire store");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    store
        .create_data_type(data_type, owned_by_id, actor_id)
        .await
        .map_err(|report| {
            // TODO: consider adding the data type, or at least its URI in the trace
            tracing::error!(error=?report, "Could not create data type");

            if report.contains::<BaseUriAlreadyExists>() {
                return StatusCode::CONFLICT;
            }

            // Insertion/update errors are considered internal server errors.
            StatusCode::INTERNAL_SERVER_ERROR
        })
        .map(Json)
}

#[utoipa::path(
    post,
    path = "/data-types/query",
    request_body = DataTypeStructuralQuery,
    tag = "DataType",
    responses(
        (status = 200, content_type = "application/json", body = Subgraph, description = "Gets a subgraph rooted at all data types that satisfy the given query, each resolved to the requested depth."),

        (status = 422, content_type = "text/plain", description = "Provided query is invalid"),
        (status = 500, description = "Store error occurred"),
    )
)]
async fn get_data_types_by_query<P: StorePool + Send>(
    pool: Extension<Arc<P>>,
    Json(query): Json<serde_json::Value>,
) -> Result<Json<Subgraph>, StatusCode> {
    pool.acquire()
        .map_err(|error| {
            tracing::error!(?error, "Could not acquire access to the store");
            StatusCode::INTERNAL_SERVER_ERROR
        })
        .and_then(|store| async move {
            let mut query = StructuralQuery::deserialize(&query).map_err(|error| {
                tracing::error!(?error, "Could not deserialize query");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            query.filter.convert_parameters().map_err(|error| {
                tracing::error!(?error, "Could not validate query");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            store.get_data_type(&query).await.map_err(|report| {
                tracing::error!(error=?report, ?query, "Could not read data types from the store");
                report_to_status_code(&report)
            })
        })
        .await
        .map(Json)
}

#[utoipa::path(
    get,
    path = "/data-types",
    tag = "DataType",
    responses(
        (status = 200, content_type = "application/json", description = "List of all data types at their latest versions", body = [PersistedDataType]),

        (status = 500, description = "Store error occurred"),
    )
)]
async fn get_latest_data_types<P: StorePool + Send>(
    pool: Extension<Arc<P>>,
) -> Result<Json<Vec<PersistedDataType>>, StatusCode> {
    read_from_store(pool.as_ref(), &Filter::<DataType>::for_latest_version())
        .await
        .map(Json)
}

#[utoipa::path(
    get,
    path = "/data-types/{uri}",
    tag = "DataType",
    responses(
        (status = 200, content_type = "application/json", description = "The schema of the requested data type", body = PersistedDataType),
        (status = 422, content_type = "text/plain", description = "Provided URI is invalid"),

        (status = 404, description = "Data type was not found"),
        (status = 500, description = "Store error occurred"),
    ),
    params(
        ("uri" = String, Path, description = "The URI of the data type"),
    )
)]
async fn get_data_type<P: StorePool + Send>(
    uri: Path<VersionedUri>,
    pool: Extension<Arc<P>>,
) -> Result<Json<PersistedDataType>, StatusCode> {
    read_from_store(
        pool.as_ref(),
        &Filter::<DataType>::for_versioned_uri(&uri.0),
    )
    .await
    .and_then(|mut data_types| data_types.pop().ok_or(StatusCode::NOT_FOUND))
    .map(Json)
}

#[derive(ToSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateDataTypeRequest {
    #[schema(value_type = VAR_UPDATE_DATA_TYPE)]
    schema: serde_json::Value,
    #[schema(value_type = String)]
    type_to_update: VersionedUri,
    actor_id: UpdatedById,
}

#[utoipa::path(
    put,
    path = "/data-types",
    tag = "DataType",
    responses(
        (status = 200, content_type = "application/json", description = "The metadata of the updated data type", body = PersistedOntologyMetadata),
        (status = 422, content_type = "text/plain", description = "Provided request body is invalid"),

        (status = 404, description = "Base data type ID was not found"),
        (status = 500, description = "Store error occurred"),
    ),
    request_body = UpdateDataTypeRequest,
)]
async fn update_data_type<P: StorePool + Send>(
    body: Json<UpdateDataTypeRequest>,
    pool: Extension<Arc<P>>,
) -> Result<Json<PersistedOntologyMetadata>, StatusCode> {
    let Json(UpdateDataTypeRequest {
        schema,
        type_to_update,
        actor_id,
    }) = body;

    let new_type_id = VersionedUri::new(
        type_to_update.base_uri().clone(),
        type_to_update.version() + 1,
    );

    let data_type = patch_id_and_parse(&new_type_id, schema).map_err(|report| {
        tracing::error!(error=?report, "Couldn't patch schema and convert to Data Type");
        StatusCode::UNPROCESSABLE_ENTITY
        // TODO - We should probably return more information to the client
        //  https://app.asana.com/0/1201095311341924/1202574350052904/f
    })?;

    let mut store = pool.acquire().await.map_err(|report| {
        tracing::error!(error=?report, "Could not acquire store");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    store
        .update_data_type(data_type, actor_id)
        .await
        .map_err(|report| {
            tracing::error!(error=?report, "Could not update data type");

            if report.contains::<BaseUriDoesNotExist>() {
                return StatusCode::NOT_FOUND;
            }

            // Insertion/update errors are considered internal server errors.
            StatusCode::INTERNAL_SERVER_ERROR
        })
        .map(Json)
}
