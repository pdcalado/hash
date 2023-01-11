import { useLazyQuery } from "@apollo/client";
import { EmbedderGraphMessageCallbacks } from "@blockprotocol/graph";
import { Subgraph, SubgraphRootTypes } from "@hashintel/hash-subgraph";
import { useCallback } from "react";

import {
  GetAllLatestEntityTypesQuery,
  GetAllLatestEntityTypesQueryVariables,
} from "../../../../graphql/api-types.gen";
import { getAllLatestEntityTypesQuery } from "../../../../graphql/queries/ontology/entity-type.queries";

export const useBlockProtocolAggregateEntityTypes = (): {
  aggregateEntityTypes: EmbedderGraphMessageCallbacks["aggregateEntityTypes"];
} => {
  const [aggregateFn] = useLazyQuery<
    GetAllLatestEntityTypesQuery,
    GetAllLatestEntityTypesQueryVariables
  >(getAllLatestEntityTypesQuery, {
    /** @todo reconsider caching. This is done for testing/demo purposes. */
    fetchPolicy: "no-cache",
  });

  const aggregateEntityTypes = useCallback<
    EmbedderGraphMessageCallbacks["aggregateEntityTypes"]
  >(
    // @ts-expect-error -- todo-0.3 implement aggregateEntityTypes in HASH
    async ({ data }) => {
      if (!data) {
        return {
          errors: [
            {
              code: "INVALID_INPUT",
              message: "'data' must be provided for aggregateEntityTypes",
            },
          ],
        };
      }

      // @ts-expect-error -- todo-0.3 update aggregation data in @blockprotocol/graph
      const { graphResolveDepths } = data;

      /**
       * @todo Add filtering to this aggregate query using structural querying.
       *   This may mean having the backend use structural querying and relaying
       *   or doing it from here.
       *   https://app.asana.com/0/1202805690238892/1202890614880643/f
       */
      const response = await aggregateFn({
        variables: {
          constrainsValuesOn: { outgoing: 255 },
          constrainsPropertiesOn: { outgoing: 255 },
          constrainsLinksOn: { outgoing: 1 },
          constrainsLinkDestinationsOn: { outgoing: 1 },
          ...graphResolveDepths,
        },
      });

      if (!response.data) {
        return {
          errors: [
            {
              code: "INVALID_INPUT",
              message: "Error calling aggregateEntityTypes",
            },
          ],
        };
      }

      return {
        /** @todo - Is there a way we can ergonomically encode this in the GraphQL type? */
        data: response.data.getAllLatestEntityTypes as Subgraph<
          SubgraphRootTypes["entityType"]
        >,
      };
    },
    [aggregateFn],
  );

  return { aggregateEntityTypes };
};
