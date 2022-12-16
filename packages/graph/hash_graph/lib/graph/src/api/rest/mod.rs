//! The Axum webserver for accessing the Graph API operations.
//!
//! Handler methods are grouped by routes that make up the REST API.

mod api_resource;
mod middleware;

mod account;
mod data_type;
mod entity;
mod entity_type;
mod property_type;
mod utoipa_typedef;

use std::{fmt::Debug, sync::Arc};

use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use error_stack::Report;
use futures::TryFutureExt;
use include_dir::{include_dir, Dir};
use tower_http::trace::TraceLayer;
use utoipa::{
    openapi::{
        self, schema, schema::RefOr, ArrayBuilder, KnownFormat, ObjectBuilder, OneOfBuilder, Ref,
        SchemaFormat, SchemaType,
    },
    Modify, OpenApi,
};

use self::{api_resource::RoutedResource, middleware::span_maker};
use crate::{
    api::rest::middleware::log_request_and_response,
    ontology::{domain_validator::DomainValidator, Selector},
    store::{crud::Read, query::Filter, QueryError, Record, StorePool},
};

static STATIC_SCHEMAS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/api/rest/json_schemas");

fn api_resources<P: StorePool + Send + 'static>() -> Vec<Router> {
    vec![
        account::AccountResource::routes::<P>(),
        data_type::DataTypeResource::routes::<P>(),
        property_type::PropertyTypeResource::routes::<P>(),
        entity_type::EntityTypeResource::routes::<P>(),
        entity::EntityResource::routes::<P>(),
    ]
}

fn api_documentation() -> Vec<openapi::OpenApi> {
    vec![
        account::AccountResource::documentation(),
        data_type::DataTypeResource::documentation(),
        property_type::PropertyTypeResource::documentation(),
        entity_type::EntityTypeResource::documentation(),
        entity::EntityResource::documentation(),
    ]
}

fn report_to_status_code<C>(report: &Report<C>) -> StatusCode {
    let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;

    if let Some(error) = report.downcast_ref::<QueryError>() {
        tracing::error!(%error, "Unable to query from data store");
        status_code = StatusCode::UNPROCESSABLE_ENTITY;
    }
    status_code
}

async fn read_from_store<'pool, 'p, P, R>(
    pool: &'pool P,
    query: &Filter<'p, R>,
) -> Result<Vec<R>, StatusCode>
where
    P: StorePool<Store<'pool>: Read<R>>,
    R: Record<QueryPath<'p>: Sync + Debug> + Send + Sync,
{
    pool.acquire()
        .map_err(|report| {
            tracing::error!(error=?report, "Could not acquire access to the store");
            StatusCode::INTERNAL_SERVER_ERROR
        })
        .and_then(|store| async move {
            // TODO: Closure is taking reference of `store` to `read()`, so the read operation
            //       needs to be awaited on. By passing through a stream we could avoid awaiting and
            //       remove the `async move` closure
            //   see https://app.asana.com/0/1202805690238892/1202923536131158/f
            Read::read(&store, query)
                .map_err(|report| {
                    // TODO: Implement `Valuable` for queries and print them here
                    tracing::error!(error=?report, ?query, "Could not read from the store");
                    report_to_status_code(&report)
                })
                .await
        })
        .await
}

pub fn rest_api_router<P: StorePool + Send + 'static>(
    store: Arc<P>,
    domain_regex: DomainValidator,
) -> Router {
    // All api resources are merged together into a super-router.
    let merged_routes = api_resources::<P>()
        .into_iter()
        .fold(Router::new(), axum::Router::merge);

    // OpenAPI documentation is also generated by merging resources
    let open_api_doc = OpenApiDocumentation::openapi();

    // super-router can then be used as any other router.
    // Make sure extensions are added at the end so they are made available to merged routers.
    // The `/api-doc` endpoints are nested as we don't want any layers or handlers for the api-doc
    merged_routes
        .layer(Extension(store))
        .layer(Extension(domain_regex))
        .layer(axum::middleware::from_fn(log_request_and_response))
        .layer(TraceLayer::new_for_http().make_span_with(span_maker))
        .nest(
            "/api-doc",
            Router::new()
                .route(
                    "/openapi.json",
                    get({
                        let doc = open_api_doc;
                        move || async { Json(doc) }
                    }),
                )
                .route("/models/*path", get(serve_static_schema)),
        )
}

#[allow(
    clippy::unused_async,
    reason = "This route does not need async capabilities, but axum requires it in trait bounds."
)]
async fn serve_static_schema(Path(path): Path<String>) -> Result<Response, StatusCode> {
    let path = path.trim_start_matches('/');

    STATIC_SCHEMAS
        .get_file(path)
        .map_or(Err(StatusCode::NOT_FOUND), |file| {
            Ok((
                [(
                    axum::http::header::CONTENT_TYPE,
                    axum::http::HeaderValue::from_static("application/json"),
                )],
                file.contents(),
            )
                .into_response())
        })
}

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Graph", description = "HASH Graph API")
    ),
    modifiers(&MergeAddon, &ExternalRefAddon, &OperationGraphTagAddon, &FilterSchemaAddon),
    components(
        schemas(
            Selector,
        )
    ),
)]
struct OpenApiDocumentation;

/// Addon to merge multiple [`OpenApi`] documents together.
///
/// [`OpenApi`]: utoipa::openapi::OpenApi
struct MergeAddon;

impl Modify for MergeAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let api_documentation = api_documentation();

        let api_components = api_documentation
            .iter()
            .cloned()
            .filter_map(|api_docs| {
                api_docs
                    .components
                    .map(|components| components.schemas.into_iter())
            })
            .flatten();

        let mut components = openapi.components.take().unwrap_or_default();
        components.schemas.extend(api_components);
        openapi.components = Some(components);

        let mut tags = openapi.tags.take().unwrap_or_default();
        tags.extend(
            api_documentation
                .iter()
                .cloned()
                .filter_map(|api_docs| api_docs.tags)
                .flatten(),
        );
        openapi.tags = Some(tags);

        openapi.paths.paths.extend(
            api_documentation
                .iter()
                .cloned()
                .flat_map(|api_docs| api_docs.paths.paths.into_iter()),
        );
    }
}

/// Addon to allow external references in schemas.
///
/// Any component that starts with `VAR_` will transform into a relative URL in the schema and
/// receive a `.json` ending.
///
/// For example the `VAR_Entity` component will be transformed into `./models/Entity.json`
struct ExternalRefAddon;

impl Modify for ExternalRefAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        for path_item in openapi.paths.paths.values_mut() {
            for operation in path_item.operations.values_mut() {
                if let Some(request_body) = &mut operation.request_body {
                    modify_component(request_body.content.values_mut());
                }

                for response in &mut operation.responses.responses.values_mut() {
                    match response {
                        RefOr::Ref(reference) => modify_reference(reference),
                        RefOr::T(response) => modify_component(response.content.values_mut()),
                    }
                }
            }
        }

        if let Some(components) = &mut openapi.components {
            for component in &mut components.schemas.values_mut() {
                modify_schema_references(component);
            }
        }
    }
}

fn modify_component<'a>(content_iter: impl IntoIterator<Item = &'a mut openapi::Content>) {
    for content in content_iter {
        modify_schema_references(&mut content.schema);
    }
}

fn modify_schema_references(schema_component: &mut RefOr<openapi::Schema>) {
    match schema_component {
        RefOr::Ref(reference) => modify_reference(reference),
        RefOr::T(schema) => match schema {
            openapi::Schema::Object(object) => object
                .properties
                .values_mut()
                .for_each(modify_schema_references),
            openapi::Schema::Array(array) => modify_schema_references(array.items.as_mut()),
            openapi::Schema::OneOf(one_of) => {
                one_of.items.iter_mut().for_each(modify_schema_references);
            }
            _ => (),
        },
    }
}

fn modify_reference(reference: &mut openapi::Ref) {
    static REF_PREFIX: &str = "#/components/schemas/VAR_";

    if reference.ref_location.starts_with(REF_PREFIX) {
        reference
            .ref_location
            .replace_range(0..REF_PREFIX.len(), "./models/");
        reference.ref_location.make_ascii_lowercase();
        reference.ref_location.push_str(".json");
    };
}

/// Append a `Graph` tag wherever a tag appears in individual routes.
///
/// When generating API clients the tags are used for grouping routes. Having the `Graph` tag on all
/// routes makes it so that every operation appear under the same `Graph` API interface.
///
/// As generators are not all created the same way, we're putting the `Graph` tag in the beginning
/// for it to take precedence. Other tags in the system are used for logical grouping of the
/// routes, which is why we don't want to entirely replace them.
struct OperationGraphTagAddon;

impl Modify for OperationGraphTagAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        let tag = "Graph";

        for path_item in openapi.paths.paths.values_mut() {
            for operation in path_item.operations.values_mut() {
                if let Some(tags) = &mut operation.tags {
                    tags.insert(0, tag.to_owned());
                }
            }
        }
    }
}

struct FilterSchemaAddon;

impl Modify for FilterSchemaAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(ref mut components) = openapi.components {
            components.schemas.insert(
                "Filter".to_owned(),
                schema::Schema::OneOf(
                    OneOfBuilder::new()
                        .item(
                            ObjectBuilder::new()
                                .title(Some("AllFilter"))
                                .property(
                                    "all",
                                    ArrayBuilder::new().items(Ref::from_schema_name("Filter")),
                                )
                                .required("all"),
                        )
                        .item(
                            ObjectBuilder::new()
                                .title(Some("AnyFilter"))
                                .property(
                                    "any",
                                    ArrayBuilder::new().items(Ref::from_schema_name("Filter")),
                                )
                                .required("any"),
                        )
                        .item(
                            ObjectBuilder::new()
                                .title(Some("NotFilter"))
                                .property("not", Ref::from_schema_name("Filter"))
                                .required("not"),
                        )
                        .item(
                            ObjectBuilder::new()
                                .title(Some("EqualFilter"))
                                .property(
                                    "equal",
                                    ArrayBuilder::new()
                                        .items(Ref::from_schema_name("FilterExpression"))
                                        .min_items(Some(2))
                                        .max_items(Some(2)),
                                )
                                .required("equal"),
                        )
                        .item(
                            ObjectBuilder::new()
                                .title(Some("NotEqualFilter"))
                                .property(
                                    "notEqual",
                                    ArrayBuilder::new()
                                        .items(Ref::from_schema_name("FilterExpression"))
                                        .min_items(Some(2))
                                        .max_items(Some(2)),
                                )
                                .required("notEqual"),
                        )
                        .build(),
                )
                .into(),
            );
            components.schemas.insert(
                "FilterExpression".to_owned(),
                schema::Schema::OneOf(
                    OneOfBuilder::new()
                        .item(
                            ObjectBuilder::new()
                                .title(Some("PathExpression"))
                                .property(
                                    "path",
                                    ArrayBuilder::new().items(
                                        OneOfBuilder::new()
                                            .item(Ref::from_schema_name("DataTypeQueryToken"))
                                            .item(Ref::from_schema_name("PropertyTypeQueryToken"))
                                            .item(Ref::from_schema_name("EntityTypeQueryToken"))
                                            .item(Ref::from_schema_name("EntityQueryToken"))
                                            .item(Ref::from_schema_name("Selector")),
                                    ),
                                )
                                .required("path"),
                        )
                        .item(
                            ObjectBuilder::new()
                                .title(Some("ParameterExpression"))
                                .property(
                                    "parameter",
                                    OneOfBuilder::new()
                                        .item(ObjectBuilder::new().schema_type(SchemaType::Boolean))
                                        .item(
                                            ObjectBuilder::new()
                                                .schema_type(SchemaType::Number)
                                                .format(Some(SchemaFormat::KnownFormat(
                                                    KnownFormat::Float,
                                                ))),
                                        )
                                        .item(ObjectBuilder::new().schema_type(SchemaType::String)),
                                )
                                .required("parameter"),
                        )
                        .build(),
                )
                .into(),
            );
        }
    }
}
