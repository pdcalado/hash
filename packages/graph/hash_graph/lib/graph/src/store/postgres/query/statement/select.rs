use std::fmt::{self, Write};

use crate::store::postgres::query::{
    expression::OrderByExpression, AliasedColumn, AliasedTable, JoinExpression, SelectExpression,
    Transpile, WhereExpression, WithExpression,
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SelectStatement<'p> {
    pub with: WithExpression<'p>,
    pub distinct: Vec<AliasedColumn<'p>>,
    pub selects: Vec<SelectExpression<'p>>,
    pub from: AliasedTable,
    pub joins: Vec<JoinExpression<'p>>,
    pub where_expression: WhereExpression<'p>,
    pub order_by_expression: OrderByExpression<'p>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Distinctness {
    Indistinct,
    Distinct,
}

impl Transpile for SelectStatement<'_> {
    fn transpile(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if !self.with.is_empty() {
            self.with.transpile(fmt)?;
            fmt.write_char('\n')?;
        }

        fmt.write_str("SELECT ")?;

        if !self.distinct.is_empty() {
            fmt.write_str("DISTINCT ON(")?;

            for (idx, column) in self.distinct.iter().enumerate() {
                if idx > 0 {
                    fmt.write_str(", ")?;
                }
                column.transpile(fmt)?;
            }
            fmt.write_str(") ")?;
        }

        for (idx, condition) in self.selects.iter().enumerate() {
            if idx > 0 {
                fmt.write_str(", ")?;
            }
            condition.transpile(fmt)?;
        }
        fmt.write_str("\nFROM ")?;
        self.from.table.transpile(fmt)?;
        fmt.write_str(" AS ")?;
        self.from.transpile(fmt)?;

        for join in &self.joins {
            fmt.write_char('\n')?;
            join.transpile(fmt)?;
        }

        if !self.where_expression.is_empty() {
            fmt.write_char('\n')?;
            self.where_expression.transpile(fmt)?;
        }

        if !self.order_by_expression.is_empty() {
            fmt.write_char('\n')?;
            self.order_by_expression.transpile(fmt)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use postgres_types::ToSql;
    use uuid::Uuid;

    use crate::{
        identifier::time::{TransactionTime, UnresolvedTimeProjection},
        knowledge::{Entity, EntityQueryPath},
        ontology::{
            DataTypeQueryPath, DataTypeWithMetadata, EntityTypeQueryPath, EntityTypeWithMetadata,
            PropertyTypeQueryPath, PropertyTypeWithMetadata,
        },
        store::{
            postgres::query::{
                test_helper::trim_whitespace, Distinctness, Ordering, PostgresRecord,
                SelectCompiler,
            },
            query::{Filter, FilterExpression, Parameter},
        },
    };

    fn test_compilation<'f, 'p: 'f, T: PostgresRecord + 'static>(
        compiler: &SelectCompiler<'f, 'p, T>,
        expected_statement: &'static str,
        expected_parameters: &[&'f dyn ToSql],
    ) {
        let (compiled_statement, compiled_parameters) = compiler.compile();

        assert_eq!(
            trim_whitespace(compiled_statement),
            trim_whitespace(expected_statement)
        );

        let compiled_parameters = compiled_parameters
            .iter()
            .map(|parameter| format!("{parameter:?}"))
            .collect::<Vec<_>>();
        let expected_parameters = expected_parameters
            .iter()
            .map(|parameter| format!("{parameter:?}"))
            .collect::<Vec<_>>();

        assert_eq!(compiled_parameters, expected_parameters);
    }

    #[test]
    fn asterisk() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        test_compilation(
            &SelectCompiler::<DataTypeWithMetadata>::with_asterisk(&time_projection),
            r#"SELECT * FROM "data_types" AS "data_types_0_0_0""#,
            &[],
        );
    }

    #[test]
    fn simple_expression() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let mut compiler = SelectCompiler::<DataTypeWithMetadata>::with_asterisk(&time_projection);
        compiler.add_filter(&Filter::Equal(
            Some(FilterExpression::Path(DataTypeQueryPath::VersionedUri)),
            Some(FilterExpression::Parameter(Parameter::Text(Cow::Borrowed(
                "https://blockprotocol.org/@blockprotocol/types/data-type/text/v/1",
            )))),
        ));
        test_compilation(
            &compiler,
            r#"
            SELECT *
            FROM "data_types" AS "data_types_0_0_0"
            WHERE "data_types_0_0_0"."schema"->>'$id' = $1
            "#,
            &[&"https://blockprotocol.org/@blockprotocol/types/data-type/text/v/1"],
        );
    }

    #[test]
    fn specific_version() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let mut compiler = SelectCompiler::<DataTypeWithMetadata>::with_asterisk(&time_projection);

        let filter = Filter::All(vec![
            Filter::Equal(
                Some(FilterExpression::Path(DataTypeQueryPath::BaseUri)),
                Some(FilterExpression::Parameter(Parameter::Text(Cow::Borrowed(
                    "https://blockprotocol.org/@blockprotocol/types/data-type/text/",
                )))),
            ),
            Filter::Equal(
                Some(FilterExpression::Path(DataTypeQueryPath::Version)),
                Some(FilterExpression::Parameter(Parameter::Number(1.0))),
            ),
        ]);
        compiler.add_filter(&filter);

        test_compilation(
            &compiler,
            r#"
            SELECT *
            FROM "data_types" AS "data_types_0_0_0"
            INNER JOIN "type_ids" AS "type_ids_0_1_0"
              ON "type_ids_0_1_0"."version_id" = "data_types_0_0_0"."version_id"
            WHERE ("type_ids_0_1_0"."base_uri" = $1) AND ("type_ids_0_1_0"."version" = $2)
            "#,
            &[
                &"https://blockprotocol.org/@blockprotocol/types/data-type/text/",
                &1.0,
            ],
        );
    }

    #[test]
    fn latest_version() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let mut compiler = SelectCompiler::<DataTypeWithMetadata>::with_asterisk(&time_projection);

        compiler.add_filter(&Filter::Equal(
            Some(FilterExpression::Path(DataTypeQueryPath::Version)),
            Some(FilterExpression::Parameter(Parameter::Text(Cow::Borrowed(
                "latest",
            )))),
        ));

        test_compilation(
            &compiler,
            r#"
            WITH "type_ids" AS (SELECT *, MAX("type_ids_0_0_0"."version") OVER (PARTITION BY "type_ids_0_0_0"."base_uri") AS "latest_version" FROM "type_ids" AS "type_ids_0_0_0")
            SELECT *
            FROM "data_types" AS "data_types_0_0_0"
            INNER JOIN "type_ids" AS "type_ids_0_1_0"
              ON "type_ids_0_1_0"."version_id" = "data_types_0_0_0"."version_id"
            WHERE "type_ids_0_1_0"."version" = "type_ids_0_1_0"."latest_version"
            "#,
            &[],
        );
    }

    #[test]
    fn not_latest_version() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let mut compiler = SelectCompiler::<DataTypeWithMetadata>::with_asterisk(&time_projection);

        compiler.add_filter(&Filter::NotEqual(
            Some(FilterExpression::Path(DataTypeQueryPath::Version)),
            Some(FilterExpression::Parameter(Parameter::Text(Cow::Borrowed(
                "latest",
            )))),
        ));

        test_compilation(
            &compiler,
            r#"
            WITH "type_ids" AS (SELECT *, MAX("type_ids_0_0_0"."version") OVER (PARTITION BY "type_ids_0_0_0"."base_uri") AS "latest_version" FROM "type_ids" AS "type_ids_0_0_0")
            SELECT *
            FROM "data_types" AS "data_types_0_0_0"
            INNER JOIN "type_ids" AS "type_ids_0_1_0"
              ON "type_ids_0_1_0"."version_id" = "data_types_0_0_0"."version_id"
            WHERE "type_ids_0_1_0"."version" != "type_ids_0_1_0"."latest_version"
            "#,
            &[],
        );
    }

    #[test]
    fn property_type_by_referenced_data_types() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let mut compiler =
            SelectCompiler::<PropertyTypeWithMetadata>::with_asterisk(&time_projection);

        compiler.add_filter(&Filter::Equal(
            Some(FilterExpression::Path(PropertyTypeQueryPath::DataTypes(
                DataTypeQueryPath::Title,
            ))),
            Some(FilterExpression::Parameter(Parameter::Text(Cow::Borrowed(
                "Text",
            )))),
        ));

        test_compilation(
            &compiler,
            r#"
            SELECT *
            FROM "property_types" AS "property_types_0_0_0"
            INNER JOIN "property_type_data_type_references" AS "property_type_data_type_references_0_1_0"
              ON "property_type_data_type_references_0_1_0"."source_property_type_version_id" = "property_types_0_0_0"."version_id"
            INNER JOIN "data_types" AS "data_types_0_2_0"
              ON "data_types_0_2_0"."version_id" = "property_type_data_type_references_0_1_0"."target_data_type_version_id"
            WHERE "data_types_0_2_0"."schema"->>'title' = $1
            "#,
            &[&"Text"],
        );

        let filter = Filter::All(vec![
            Filter::Equal(
                Some(FilterExpression::Path(PropertyTypeQueryPath::DataTypes(
                    DataTypeQueryPath::BaseUri,
                ))),
                Some(FilterExpression::Parameter(Parameter::Text(Cow::Borrowed(
                    "https://blockprotocol.org/@blockprotocol/types/data-type/text/",
                )))),
            ),
            Filter::Equal(
                Some(FilterExpression::Path(PropertyTypeQueryPath::DataTypes(
                    DataTypeQueryPath::Version,
                ))),
                Some(FilterExpression::Parameter(Parameter::Number(1.0))),
            ),
        ]);
        compiler.add_filter(&filter);

        test_compilation(
            &compiler,
            r#"
            SELECT *
            FROM "property_types" AS "property_types_0_0_0"
            INNER JOIN "property_type_data_type_references" AS "property_type_data_type_references_0_1_0"
              ON "property_type_data_type_references_0_1_0"."source_property_type_version_id" = "property_types_0_0_0"."version_id"
            INNER JOIN "data_types" AS "data_types_0_2_0"
              ON "data_types_0_2_0"."version_id" = "property_type_data_type_references_0_1_0"."target_data_type_version_id"
            INNER JOIN "property_type_data_type_references" AS "property_type_data_type_references_1_1_0"
              ON "property_type_data_type_references_1_1_0"."source_property_type_version_id" = "property_types_0_0_0"."version_id"
            INNER JOIN "type_ids" AS "type_ids_1_2_0"
              ON "type_ids_1_2_0"."version_id" = "property_type_data_type_references_1_1_0"."target_data_type_version_id"
            WHERE "data_types_0_2_0"."schema"->>'title' = $1
              AND ("type_ids_1_2_0"."base_uri" = $2) AND ("type_ids_1_2_0"."version" = $3)
            "#,
            &[
                &"Text",
                &"https://blockprotocol.org/@blockprotocol/types/data-type/text/",
                &1.0,
            ],
        );
    }

    #[test]
    fn property_type_by_referenced_property_types() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let mut compiler =
            SelectCompiler::<PropertyTypeWithMetadata>::with_asterisk(&time_projection);

        let filter = Filter::Equal(
            Some(FilterExpression::Path(
                PropertyTypeQueryPath::PropertyTypes(Box::new(PropertyTypeQueryPath::Title)),
            )),
            Some(FilterExpression::Parameter(Parameter::Text(Cow::Borrowed(
                "Text",
            )))),
        );
        compiler.add_filter(&filter);

        test_compilation(
            &compiler,
            r#"
            SELECT *
            FROM "property_types" AS "property_types_0_0_0"
            INNER JOIN "property_type_property_type_references" AS "property_type_property_type_references_0_1_0"
              ON "property_type_property_type_references_0_1_0"."source_property_type_version_id" = "property_types_0_0_0"."version_id"
            INNER JOIN "property_types" AS "property_types_0_2_0"
              ON "property_types_0_2_0"."version_id" = "property_type_property_type_references_0_1_0"."target_property_type_version_id"
            WHERE "property_types_0_2_0"."schema"->>'title' = $1
            "#,
            &[&"Text"],
        );
    }

    #[test]
    fn entity_type_by_referenced_property_types() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let mut compiler =
            SelectCompiler::<EntityTypeWithMetadata>::with_asterisk(&time_projection);

        let filter = Filter::Equal(
            Some(FilterExpression::Path(EntityTypeQueryPath::Properties(
                PropertyTypeQueryPath::Title,
            ))),
            Some(FilterExpression::Parameter(Parameter::Text(Cow::Borrowed(
                "Name",
            )))),
        );
        compiler.add_filter(&filter);

        test_compilation(
            &compiler,
            r#"
            SELECT *
            FROM "entity_types" AS "entity_types_0_0_0"
            INNER JOIN "entity_type_property_type_references" AS "entity_type_property_type_references_0_1_0"
              ON "entity_type_property_type_references_0_1_0"."source_entity_type_version_id" = "entity_types_0_0_0"."version_id"
            INNER JOIN "property_types" AS "property_types_0_2_0"
              ON "property_types_0_2_0"."version_id" = "entity_type_property_type_references_0_1_0"."target_property_type_version_id"
            WHERE "property_types_0_2_0"."schema"->>'title' = $1
            "#,
            &[&"Name"],
        );
    }

    #[test]
    fn entity_type_by_referenced_link_types() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let mut compiler =
            SelectCompiler::<EntityTypeWithMetadata>::with_asterisk(&time_projection);

        let filter = Filter::Equal(
            Some(FilterExpression::Path(EntityTypeQueryPath::Links(
                Box::new(EntityTypeQueryPath::Links(Box::new(
                    EntityTypeQueryPath::Title,
                ))),
            ))),
            Some(FilterExpression::Parameter(Parameter::Text(Cow::Borrowed(
                "Friend Of",
            )))),
        );
        compiler.add_filter(&filter);

        test_compilation(
            &compiler,
            r#"
            SELECT *
            FROM "entity_types" AS "entity_types_0_0_0"
            INNER JOIN "entity_type_entity_type_references" AS "entity_type_entity_type_references_0_1_0"
              ON "entity_type_entity_type_references_0_1_0"."source_entity_type_version_id" = "entity_types_0_0_0"."version_id"
            INNER JOIN "entity_types" AS "entity_types_0_2_0"
              ON "entity_types_0_2_0"."version_id" = "entity_type_entity_type_references_0_1_0"."target_entity_type_version_id"
            INNER JOIN "entity_type_entity_type_references" AS "entity_type_entity_type_references_0_3_0"
              ON "entity_type_entity_type_references_0_3_0"."source_entity_type_version_id" = "entity_types_0_2_0"."version_id"
            INNER JOIN "entity_types" AS "entity_types_0_4_0"
              ON "entity_types_0_4_0"."version_id" = "entity_type_entity_type_references_0_3_0"."target_entity_type_version_id"
            WHERE jsonb_extract_path("entity_types_0_0_0"."schema", 'links', "entity_types_0_2_0"."schema"->>'$id') IS NOT NULL
              AND jsonb_extract_path("entity_types_0_2_0"."schema", 'links', "entity_types_0_4_0"."schema"->>'$id') IS NOT NULL
              AND "entity_types_0_4_0"."schema"->>'title' = $1
            "#,
            &[&"Friend Of"],
        );
    }

    #[test]
    fn entity_type_by_inheritance() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let mut compiler =
            SelectCompiler::<EntityTypeWithMetadata>::with_asterisk(&time_projection);

        let filter = Filter::Equal(
            Some(FilterExpression::Path(EntityTypeQueryPath::InheritsFrom(
                Box::new(EntityTypeQueryPath::BaseUri),
            ))),
            Some(FilterExpression::Parameter(Parameter::Text(Cow::Borrowed(
                "https://blockprotocol.org/@blockprotocol/types/entity-type/link/",
            )))),
        );
        compiler.add_filter(&filter);

        test_compilation(
            &compiler,
            r#"
            SELECT *
            FROM "entity_types" AS "entity_types_0_0_0"
            INNER JOIN "entity_type_entity_type_references" AS "entity_type_entity_type_references_0_1_0"
              ON "entity_type_entity_type_references_0_1_0"."source_entity_type_version_id" = "entity_types_0_0_0"."version_id"
            INNER JOIN "entity_types" AS "entity_types_0_2_0"
              ON "entity_types_0_2_0"."version_id" = "entity_type_entity_type_references_0_1_0"."target_entity_type_version_id"
            INNER JOIN "type_ids" AS "type_ids_0_3_0"
              ON "type_ids_0_3_0"."version_id" = "entity_types_0_2_0"."version_id"
            WHERE jsonb_contains("entity_types_0_0_0"."schema"->'allOf', jsonb_build_array(jsonb_build_object('$ref', "entity_types_0_2_0"."schema"->>'$id'))) IS NOT NULL
              AND "type_ids_0_3_0"."base_uri" = $1
            "#,
            &[&"https://blockprotocol.org/@blockprotocol/types/entity-type/link/"],
        );
    }

    #[test]
    fn entity_simple_query() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let kernel = time_projection.kernel().cast::<TransactionTime>();
        let mut compiler = SelectCompiler::<Entity>::with_asterisk(&time_projection);

        let filter = Filter::Equal(
            Some(FilterExpression::Path(EntityQueryPath::Uuid)),
            Some(FilterExpression::Parameter(Parameter::Text(Cow::Borrowed(
                "12345678-ABCD-4321-5678-ABCD5555DCBA",
            )))),
        );
        compiler.add_filter(&filter);

        test_compilation(
            &compiler,
            r#"
            SELECT *
            FROM "entities" AS "entities_0_0_0"
            WHERE "entities_0_0_0"."transaction_time" @> $1::TIMESTAMPTZ
              AND "entities_0_0_0"."entity_uuid" = $2
            "#,
            &[&kernel, &"12345678-ABCD-4321-5678-ABCD5555DCBA"],
        );
    }

    #[test]
    fn entity_latest_version_query() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let kernel = time_projection.kernel().cast::<TransactionTime>();
        let mut compiler = SelectCompiler::<Entity>::with_asterisk(&time_projection);

        let filter = Filter::Equal(
            Some(FilterExpression::Path(EntityQueryPath::ProjectedTime)),
            Some(FilterExpression::Parameter(Parameter::Text(Cow::Borrowed(
                "latest",
            )))),
        );
        compiler.add_filter(&filter);

        test_compilation(
            &compiler,
            r#"
            SELECT *
            FROM "entities" AS "entities_0_0_0"
            WHERE "entities_0_0_0"."transaction_time" @> $1::TIMESTAMPTZ
              AND "entities_0_0_0"."decision_time" @> now()::TIMESTAMPTZ
            "#,
            &[&kernel],
        );
    }

    #[test]
    fn entity_with_manual_selection() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let kernel = time_projection.kernel().cast::<TransactionTime>();
        let mut compiler = SelectCompiler::<Entity>::new(&time_projection);
        compiler.add_distinct_selection_with_ordering(
            &EntityQueryPath::Uuid,
            Distinctness::Distinct,
            Some(Ordering::Ascending),
        );
        compiler.add_distinct_selection_with_ordering(
            &EntityQueryPath::DecisionTime,
            Distinctness::Distinct,
            Some(Ordering::Descending),
        );
        compiler.add_selection_path(&EntityQueryPath::Properties(None));

        let filter = Filter::Equal(
            Some(FilterExpression::Path(EntityQueryPath::UpdatedById)),
            Some(FilterExpression::Parameter(Parameter::Uuid(Uuid::nil()))),
        );
        compiler.add_filter(&filter);

        test_compilation(
            &compiler,
            r#"
            SELECT
                DISTINCT ON("entities_0_0_0"."entity_uuid", "entities_0_0_0"."decision_time")
                "entities_0_0_0"."entity_uuid",
                "entities_0_0_0"."decision_time",
                "entities_0_0_0"."properties"
            FROM "entities" AS "entities_0_0_0"
            WHERE "entities_0_0_0"."transaction_time" @> $1::TIMESTAMPTZ
              AND "entities_0_0_0"."updated_by_id" = $2
            ORDER BY "entities_0_0_0"."entity_uuid" ASC,
                     "entities_0_0_0"."decision_time" DESC
            "#,
            &[&kernel, &Uuid::nil()],
        );
    }

    #[test]
    fn entity_property_query() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let kernel = time_projection.kernel().cast::<TransactionTime>();
        let mut compiler = SelectCompiler::<Entity>::with_asterisk(&time_projection);

        let filter = Filter::Equal(
            Some(FilterExpression::Path(EntityQueryPath::Properties(Some(
                Cow::Borrowed("https://blockprotocol.org/@alice/types/property-type/name/"),
            )))),
            Some(FilterExpression::Parameter(Parameter::Text(Cow::Borrowed(
                "Bob",
            )))),
        );
        compiler.add_filter(&filter);

        test_compilation(
            &compiler,
            r#"
            SELECT *
            FROM "entities" AS "entities_0_0_0"
            WHERE "entities_0_0_0"."transaction_time" @> $2::TIMESTAMPTZ
              AND "entities_0_0_0"."properties"->>$1 = $3
            "#,
            &[
                &"https://blockprotocol.org/@alice/types/property-type/name/",
                &kernel,
                &"Bob",
            ],
        );
    }

    #[test]
    fn entity_property_null_query() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let kernel = time_projection.kernel().cast::<TransactionTime>();
        let mut compiler = SelectCompiler::<Entity>::with_asterisk(&time_projection);

        let filter = Filter::Equal(
            Some(FilterExpression::Path(EntityQueryPath::Properties(Some(
                Cow::Borrowed("https://blockprotocol.org/@alice/types/property-type/name/"),
            )))),
            None,
        );
        compiler.add_filter(&filter);

        test_compilation(
            &compiler,
            r#"
            SELECT *
            FROM "entities" AS "entities_0_0_0"
            WHERE "entities_0_0_0"."transaction_time" @> $2::TIMESTAMPTZ
              AND "entities_0_0_0"."properties"->>$1 IS NULL
            "#,
            &[
                &"https://blockprotocol.org/@alice/types/property-type/name/",
                &kernel,
            ],
        );
    }

    #[test]
    fn entity_outgoing_link_query() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let kernel = time_projection.kernel().cast::<TransactionTime>();
        let mut compiler = SelectCompiler::<Entity>::with_asterisk(&time_projection);

        let filter = Filter::Equal(
            Some(FilterExpression::Path(EntityQueryPath::OutgoingLinks(
                Box::new(EntityQueryPath::RightEntity(Box::new(
                    EntityQueryPath::ProjectedTime,
                ))),
            ))),
            Some(FilterExpression::Parameter(Parameter::Text(Cow::Borrowed(
                "latest",
            )))),
        );
        compiler.add_filter(&filter);

        test_compilation(
            &compiler,
            r#"
            SELECT *
            FROM "entities" AS "entities_0_0_0"
            LEFT OUTER JOIN "entities" AS "entities_0_1_0"
              ON "entities_0_1_0"."left_entity_uuid" = "entities_0_0_0"."entity_uuid"
            RIGHT OUTER JOIN "entities" AS "entities_0_2_0"
              ON "entities_0_2_0"."entity_uuid" = "entities_0_1_0"."right_entity_uuid"
            WHERE "entities_0_0_0"."transaction_time" @> $1::TIMESTAMPTZ
              AND "entities_0_1_0"."transaction_time" @> $1::TIMESTAMPTZ
              AND "entities_0_2_0"."transaction_time" @> $1::TIMESTAMPTZ
              AND "entities_0_2_0"."decision_time" @> now()::TIMESTAMPTZ
            "#,
            &[&kernel],
        );
    }

    #[test]
    fn entity_incoming_link_query() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let kernel = time_projection.kernel().cast::<TransactionTime>();
        let mut compiler = SelectCompiler::<Entity>::with_asterisk(&time_projection);

        let filter = Filter::Equal(
            Some(FilterExpression::Path(EntityQueryPath::IncomingLinks(
                Box::new(EntityQueryPath::LeftEntity(Box::new(
                    EntityQueryPath::ProjectedTime,
                ))),
            ))),
            Some(FilterExpression::Parameter(Parameter::Text(Cow::Borrowed(
                "latest",
            )))),
        );
        compiler.add_filter(&filter);

        test_compilation(
            &compiler,
            r#"
            SELECT *
            FROM "entities" AS "entities_0_0_0"
            LEFT OUTER JOIN "entities" AS "entities_0_1_0"
              ON "entities_0_1_0"."right_entity_uuid" = "entities_0_0_0"."entity_uuid"
            RIGHT OUTER JOIN "entities" AS "entities_0_2_0"
              ON "entities_0_2_0"."entity_uuid" = "entities_0_1_0"."left_entity_uuid"
            WHERE "entities_0_0_0"."transaction_time" @> $1::TIMESTAMPTZ
              AND "entities_0_1_0"."transaction_time" @> $1::TIMESTAMPTZ
              AND "entities_0_2_0"."transaction_time" @> $1::TIMESTAMPTZ
              AND "entities_0_2_0"."decision_time" @> now()::TIMESTAMPTZ
            "#,
            &[&kernel],
        );
    }

    #[test]
    fn link_entity_left_right_id() {
        let time_projection = UnresolvedTimeProjection::default().resolve();
        let kernel = time_projection.kernel().cast::<TransactionTime>();
        let mut compiler = SelectCompiler::<Entity>::with_asterisk(&time_projection);

        let filter = Filter::All(vec![
            Filter::Equal(
                Some(FilterExpression::Path(EntityQueryPath::LeftEntity(
                    Box::new(EntityQueryPath::Uuid),
                ))),
                Some(FilterExpression::Parameter(Parameter::Uuid(Uuid::nil()))),
            ),
            Filter::Equal(
                Some(FilterExpression::Path(EntityQueryPath::LeftEntity(
                    Box::new(EntityQueryPath::OwnedById),
                ))),
                Some(FilterExpression::Parameter(Parameter::Uuid(Uuid::nil()))),
            ),
            Filter::Equal(
                Some(FilterExpression::Path(EntityQueryPath::RightEntity(
                    Box::new(EntityQueryPath::Uuid),
                ))),
                Some(FilterExpression::Parameter(Parameter::Uuid(Uuid::nil()))),
            ),
            Filter::Equal(
                Some(FilterExpression::Path(EntityQueryPath::RightEntity(
                    Box::new(EntityQueryPath::OwnedById),
                ))),
                Some(FilterExpression::Parameter(Parameter::Uuid(Uuid::nil()))),
            ),
        ]);
        compiler.add_filter(&filter);

        test_compilation(
            &compiler,
            r#"
            SELECT *
            FROM "entities" AS "entities_0_0_0"
            WHERE "entities_0_0_0"."transaction_time" @> $1::TIMESTAMPTZ
              AND ("entities_0_0_0"."left_entity_uuid" = $2)
              AND ("entities_0_0_0"."left_owned_by_id" = $3)
              AND ("entities_0_0_0"."right_entity_uuid" = $4)
              AND ("entities_0_0_0"."right_owned_by_id" = $5)
            "#,
            &[
                &kernel,
                &Uuid::nil(),
                &Uuid::nil(),
                &Uuid::nil(),
                &Uuid::nil(),
            ],
        );
    }

    mod predefined {
        use type_system::uri::{BaseUri, VersionedUri};

        use super::*;
        use crate::{
            identifier::{
                account::AccountId,
                knowledge::EntityId,
                ontology::{OntologyTypeEditionId, OntologyTypeVersion},
                time::Timestamp,
                EntityVertexId,
            },
            knowledge::EntityUuid,
            provenance::OwnedById,
        };

        #[test]
        fn for_versioned_uri() {
            let uri = VersionedUri::new(
                BaseUri::new(
                    "https://blockprotocol.org/@blockprotocol/types/data-type/text/".to_owned(),
                )
                .expect("invalid base uri"),
                1,
            );

            let time_projection = UnresolvedTimeProjection::default().resolve();
            let mut compiler =
                SelectCompiler::<DataTypeWithMetadata>::with_asterisk(&time_projection);

            let filter = Filter::for_versioned_uri(&uri);
            compiler.add_filter(&filter);

            test_compilation(
                &compiler,
                r#"
                SELECT *
                FROM "data_types" AS "data_types_0_0_0"
                INNER JOIN "type_ids" AS "type_ids_0_1_0"
                  ON "type_ids_0_1_0"."version_id" = "data_types_0_0_0"."version_id"
                WHERE ("type_ids_0_1_0"."base_uri" = $1) AND ("type_ids_0_1_0"."version" = $2)
                "#,
                &[&uri.base_uri().as_str(), &i64::from(uri.version())],
            );
        }

        #[test]
        fn for_ontology_type_edition_id() {
            let uri = OntologyTypeEditionId::new(
                BaseUri::new(
                    "https://blockprotocol.org/@blockprotocol/types/data-type/text/".to_owned(),
                )
                .expect("invalid base uri"),
                OntologyTypeVersion::new(1),
            );

            let time_projection = UnresolvedTimeProjection::default().resolve();
            let mut compiler =
                SelectCompiler::<DataTypeWithMetadata>::with_asterisk(&time_projection);

            let filter = Filter::for_ontology_type_edition_id(&uri);
            compiler.add_filter(&filter);

            test_compilation(
                &compiler,
                r#"
                SELECT *
                FROM "data_types" AS "data_types_0_0_0"
                INNER JOIN "type_ids" AS "type_ids_0_1_0"
                  ON "type_ids_0_1_0"."version_id" = "data_types_0_0_0"."version_id"
                WHERE ("type_ids_0_1_0"."base_uri" = $1) AND ("type_ids_0_1_0"."version" = $2)
                "#,
                &[&uri.base_id().as_str(), &i64::from(uri.version().inner())],
            );
        }

        #[test]
        fn for_entity_by_entity_id() {
            let entity_id = EntityId::new(
                OwnedById::new(AccountId::new(Uuid::new_v4())),
                EntityUuid::new(Uuid::new_v4()),
            );

            let time_projection = UnresolvedTimeProjection::default().resolve();
            let kernel = time_projection.kernel().cast::<TransactionTime>();
            let mut compiler = SelectCompiler::<Entity>::with_asterisk(&time_projection);

            let filter = Filter::for_entity_by_entity_id(entity_id);
            compiler.add_filter(&filter);

            test_compilation(
                &compiler,
                r#"
                SELECT *
                FROM "entities" AS "entities_0_0_0"
                WHERE "entities_0_0_0"."transaction_time" @> $1::TIMESTAMPTZ
                  AND ("entities_0_0_0"."owned_by_id" = $2)
                  AND ("entities_0_0_0"."entity_uuid" = $3)
                "#,
                &[
                    &kernel,
                    &entity_id.owned_by_id().as_uuid(),
                    &entity_id.entity_uuid().as_uuid(),
                ],
            );
        }

        #[test]
        fn for_entity_by_edition_id() {
            let entity_vertex_id = EntityVertexId::new(
                EntityId::new(
                    OwnedById::new(AccountId::new(Uuid::new_v4())),
                    EntityUuid::new(Uuid::new_v4()),
                ),
                Timestamp::now(),
            );

            let time_projection = UnresolvedTimeProjection::default().resolve();
            let kernel = time_projection.kernel().cast::<TransactionTime>();
            let mut compiler = SelectCompiler::<Entity>::with_asterisk(&time_projection);

            let filter = Filter::for_entity_by_vertex_id(entity_vertex_id);
            compiler.add_filter(&filter);

            test_compilation(
                &compiler,
                r#"
                SELECT *
                FROM "entities" AS "entities_0_0_0"
                WHERE "entities_0_0_0"."transaction_time" @> $1::TIMESTAMPTZ
                  AND ("entities_0_0_0"."owned_by_id" = $2)
                  AND ("entities_0_0_0"."entity_uuid" = $3)
                  AND (lower("entities_0_0_0"."decision_time") = $4)
                "#,
                &[
                    &kernel,
                    &entity_vertex_id.base_id().owned_by_id().as_uuid(),
                    &entity_vertex_id.base_id().entity_uuid().as_uuid(),
                    &entity_vertex_id.version(),
                ],
            );
        }

        #[test]
        fn for_outgoing_link_by_source_entity_vertex_id() {
            let entity_vertex_id = EntityVertexId::new(
                EntityId::new(
                    OwnedById::new(AccountId::new(Uuid::new_v4())),
                    EntityUuid::new(Uuid::new_v4()),
                ),
                Timestamp::now(),
            );

            let time_projection = UnresolvedTimeProjection::default().resolve();
            let kernel = time_projection.kernel().cast::<TransactionTime>();
            let mut compiler = SelectCompiler::<Entity>::with_asterisk(&time_projection);

            let filter = Filter::for_outgoing_link_by_source_entity_vertex_id(entity_vertex_id);
            compiler.add_filter(&filter);

            test_compilation(
                &compiler,
                r#"
                SELECT *
                FROM "entities" AS "entities_0_0_0"
                RIGHT OUTER JOIN "entities" AS "entities_0_1_0"
                  ON "entities_0_1_0"."entity_uuid" = "entities_0_0_0"."left_entity_uuid"
                WHERE "entities_0_0_0"."transaction_time" @> $1::TIMESTAMPTZ
                  AND "entities_0_1_0"."transaction_time" @> $1::TIMESTAMPTZ
                  AND ("entities_0_0_0"."left_owned_by_id" = $2)
                  AND ("entities_0_0_0"."left_entity_uuid" = $3)
                  AND (lower("entities_0_1_0"."decision_time") = $4)
                "#,
                &[
                    &kernel,
                    &entity_vertex_id.base_id().owned_by_id().as_uuid(),
                    &entity_vertex_id.base_id().entity_uuid().as_uuid(),
                    &entity_vertex_id.version(),
                ],
            );
        }

        #[test]
        fn for_left_entity_by_entity_vertex_id() {
            let entity_vertex_id = EntityVertexId::new(
                EntityId::new(
                    OwnedById::new(AccountId::new(Uuid::new_v4())),
                    EntityUuid::new(Uuid::new_v4()),
                ),
                Timestamp::now(),
            );

            let time_projection = UnresolvedTimeProjection::default().resolve();
            let kernel = time_projection.kernel().cast::<TransactionTime>();
            let mut compiler = SelectCompiler::<Entity>::with_asterisk(&time_projection);

            let filter = Filter::for_left_entity_by_entity_vertex_id(entity_vertex_id);
            compiler.add_filter(&filter);

            test_compilation(
                &compiler,
                r#"
                SELECT *
                FROM "entities" AS "entities_0_0_0"
                LEFT OUTER JOIN "entities" AS "entities_0_1_0"
                  ON "entities_0_1_0"."left_entity_uuid" = "entities_0_0_0"."entity_uuid"
                WHERE "entities_0_0_0"."transaction_time" @> $1::TIMESTAMPTZ
                  AND "entities_0_1_0"."transaction_time" @> $1::TIMESTAMPTZ
                  AND ("entities_0_1_0"."owned_by_id" = $2)
                  AND ("entities_0_1_0"."entity_uuid" = $3)
                  AND (lower("entities_0_1_0"."decision_time") = $4)
                "#,
                &[
                    &kernel,
                    &entity_vertex_id.base_id().owned_by_id().as_uuid(),
                    &entity_vertex_id.base_id().entity_uuid().as_uuid(),
                    &entity_vertex_id.version(),
                ],
            );
        }

        #[test]
        fn for_right_entity_by_entity_vertex_id() {
            let entity_vertex_id = EntityVertexId::new(
                EntityId::new(
                    OwnedById::new(AccountId::new(Uuid::new_v4())),
                    EntityUuid::new(Uuid::new_v4()),
                ),
                Timestamp::now(),
            );

            let time_projection = UnresolvedTimeProjection::default().resolve();
            let kernel = time_projection.kernel().cast::<TransactionTime>();
            let mut compiler = SelectCompiler::<Entity>::with_asterisk(&time_projection);

            let filter = Filter::for_right_entity_by_entity_vertex_id(entity_vertex_id);
            compiler.add_filter(&filter);

            test_compilation(
                &compiler,
                r#"
                SELECT *
                FROM "entities" AS "entities_0_0_0"
                LEFT OUTER JOIN "entities" AS "entities_0_1_0"
                  ON "entities_0_1_0"."right_entity_uuid" = "entities_0_0_0"."entity_uuid"
                WHERE "entities_0_0_0"."transaction_time" @> $1::TIMESTAMPTZ
                  AND "entities_0_1_0"."transaction_time" @> $1::TIMESTAMPTZ
                  AND ("entities_0_1_0"."owned_by_id" = $2)
                  AND ("entities_0_1_0"."entity_uuid" = $3)
                  AND (lower("entities_0_1_0"."decision_time") = $4)
                "#,
                &[
                    &kernel,
                    &entity_vertex_id.base_id().owned_by_id().as_uuid(),
                    &entity_vertex_id.base_id().entity_uuid().as_uuid(),
                    &entity_vertex_id.version(),
                ],
            );
        }
    }
}
