import { useLazyQuery } from "@apollo/client";
import { EmbedderGraphMessageCallbacks } from "@blockprotocol/graph";
import { EntityId } from "@hashintel/hash-shared/types";
import { Subgraph, SubgraphRootTypes } from "@hashintel/hash-subgraph";
import { useCallback } from "react";

import {
  GetEntityQuery,
  GetEntityQueryVariables,
} from "../../../../graphql/api-types.gen";
import { getEntityQuery } from "../../../../graphql/queries/knowledge/entity.queries";

export const useBlockProtocolGetEntity = (): {
  getEntity: EmbedderGraphMessageCallbacks["getEntity"];
} => {
  const [getEntityFn] = useLazyQuery<GetEntityQuery, GetEntityQueryVariables>(
    getEntityQuery,
    {
      /** @todo reconsider caching. This is done for testing/demo purposes. */
      fetchPolicy: "no-cache",
    },
  );

  const getEntity = useCallback<EmbedderGraphMessageCallbacks["getEntity"]>(
    // @ts-expect-error todo-0.3 fix mismatch between EntityId in @blockprotocol/graph and HASH
    async ({ data }) => {
      if (!data) {
        return {
          errors: [
            {
              code: "INVALID_INPUT",
              message: "'data' must be provided for getEntity",
            },
          ],
        };
      }

      const { entityId, graphResolveDepths } = data;

      const { data: response } = await getEntityFn({
        variables: {
          entityId: entityId as EntityId, // @todo-0.3 consider validating that this matches the id format
          constrainsValuesOn: { outgoing: 255 },
          constrainsPropertiesOn: { outgoing: 255 },
          constrainsLinksOn: { outgoing: 1 },
          constrainsLinkDestinationsOn: { outgoing: 1 },
          isOfType: { outgoing: 1 },
          hasLeftEntity: { outgoing: 1, incoming: 1 },
          hasRightEntity: { outgoing: 1, incoming: 1 },
          ...graphResolveDepths,
        },
      });

      if (!response) {
        return {
          errors: [
            {
              code: "INVALID_INPUT",
              message: "Error calling getEntity",
            },
          ],
        };
      }

      return {
        /** @todo - Is there a way we can ergonomically encode this in the GraphQL type? */
        data: response.getEntity as Subgraph<SubgraphRootTypes["entity"]>,
      };
    },
    [getEntityFn],
  );

  return { getEntity };
};
