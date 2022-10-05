import { useEffect, useMemo, useState } from "react";
import { useRouter } from "next/router";
import { Container, Typography } from "@mui/material";
import init, { ValueOrArray } from "@blockprotocol/type-system";

import { useUser } from "../components/hooks/useUser";
import { NextPageWithLayout } from "../shared/layout";
import { useBlockProtocolFunctionsWithOntology } from "./type-editor/blockprotocol-ontology-functions-hook";
import { EntityResponse } from "../components/hooks/blockProtocolFunctions/knowledge/knowledge-shim";

/**
 * Helper type-guard for determining if a `ValueOrArray` definition is an array or a value.
 */
const isArrayDefinition = <T,>(
  input: ValueOrArray<T>,
): input is ValueOrArray.Array<T> => "type" in input && input.type === "array";

/**
 * This component is an example usage of the `getEntity` BP function.
 * This is meant to be removed as soon as it's unneeded.
 */
const ExampleUsage = ({ ownedById }: { ownedById: string }) => {
  const { user } = useUser();
  const [entity, setEntity] = useState<EntityResponse>();

  const { getEntity } = useBlockProtocolFunctionsWithOntology(ownedById);

  useEffect(() => {
    if (user) {
      // As an example entity, we are going to use the currently logged in user's entity ID
      const entityId = user.entityId;

      void getEntity({ data: { entityId } }).then(({ data }) => {
        setEntity(data);
      });
    }
  }, [user, getEntity]);

  const { entityTypeRootedSubgraph, ...entityWithoutEntityType } = entity ?? {};

  const { entityType } = entityTypeRootedSubgraph ?? {};

  // The (top-level) property type IDs defined in the entity type
  const propertyTypeIds = useMemo(
    () =>
      entityType
        ? Object.values(entityType.properties).map((value) =>
            isArrayDefinition(value) ? value.items.$ref : value.$ref,
          )
        : undefined,
    [entityType],
  );

  // The (top-level) property type definitions, referenced in the entity type
  const propertyTypeDefinitions = useMemo(
    () =>
      entityTypeRootedSubgraph && propertyTypeIds
        ? entityTypeRootedSubgraph.referencedPropertyTypes.filter(
            ({ propertyTypeId }) => propertyTypeIds.includes(propertyTypeId),
          )
        : undefined,
    [entityTypeRootedSubgraph, propertyTypeIds],
  );

  return (
    <Container>
      <Typography>Entity</Typography>
      <pre style={{ overflowX: "scroll" }}>
        {JSON.stringify(entityWithoutEntityType ?? {}, null, 2)}
      </pre>
      <Typography>Entity type</Typography>
      <pre style={{ overflowX: "scroll" }}>
        {JSON.stringify(entityType ?? {}, null, 2)}
      </pre>
      <Typography>Top-level property type definitions</Typography>
      <pre style={{ overflowX: "scroll" }}>
        {JSON.stringify(propertyTypeDefinitions ?? {}, null, 2)}
      </pre>
    </Container>
  );
};

const ExampleEntityEditorPage: NextPageWithLayout = () => {
  const router = useRouter();
  // The user is important to allow using Block Protocol functions
  // such as: `const functions = useBlockProtocolFunctionsWithOntology(user.accountId);`
  const { user, loading: loadingUser, kratosSession } = useUser();
  const [loadingTypeSystem, setLoadingTypeSystem] = useState(true);

  useEffect(() => {
    if (loadingTypeSystem) {
      void (async () => {
        await init().then(() => {
          setLoadingTypeSystem(false);
        });
      })();
    }
  }, [loadingTypeSystem, setLoadingTypeSystem]);

  useEffect(() => {
    if (!loadingUser && !kratosSession) {
      void router.push("/login");
    }
  }, [loadingUser, router, kratosSession]);

  return loadingUser || !user || loadingTypeSystem ? (
    <Container sx={{ pt: 10 }}>Loading...</Container>
  ) : (
    <Container sx={{ pt: 10 }}>
      <ExampleUsage ownedById={user.accountId} />
    </Container>
  );
};

export default ExampleEntityEditorPage;
