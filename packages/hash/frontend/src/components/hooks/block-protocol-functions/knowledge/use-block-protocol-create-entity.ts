import { useMutation } from "@apollo/client";
import { EmbedderGraphMessageCallbacks } from "@blockprotocol/graph";
import { OwnedById } from "@hashintel/hash-shared/types";
import { useCallback } from "react";

import {
  CreateEntityMutation,
  CreateEntityMutationVariables,
} from "../../../../graphql/api-types.gen";
import { createEntityMutation } from "../../../../graphql/queries/knowledge/entity.queries";

export const useBlockProtocolCreateEntity = (
  ownedById: OwnedById | null,
  readonly?: boolean,
): {
  createEntity: EmbedderGraphMessageCallbacks["createEntity"];
} => {
  const [createFn] = useMutation<
    CreateEntityMutation,
    CreateEntityMutationVariables
  >(createEntityMutation, {
    /** @todo reconsider caching. This is done for testing/demo purposes. */
    fetchPolicy: "no-cache",
  });

  // @ts-expect-error todo-0.3 fix mismatch between EntityId in @blockprotocol/graph and HASH
  const createEntity: EmbedderGraphMessageCallbacks["createEntity"] =
    useCallback(
      async ({ data }) => {
        if (readonly) {
          return {
            errors: [
              {
                code: "FORBIDDEN",
                message: "Operation can't be carried out in readonly mode",
              },
            ],
          };
        }

        if (!ownedById) {
          throw new Error(
            "Hook was constructed without `ownedById` while not in readonly mode. Data must be created under an account.",
          );
        }

        if (!data) {
          return {
            errors: [
              {
                code: "INVALID_INPUT",
                message: "'data' must be provided for createEntity",
              },
            ],
          };
        }

        const { entityTypeId, properties, linkData } = data;

        const { data: createEntityResponseData } = await createFn({
          variables: {
            entityTypeId,
            ownedById,
            properties,
            // @ts-expect-error todo-0.3 assert as branded ids as safely as possible (don't assert whole object, only the ids)
            linkData,
          },
        });

        const { createEntity: createdEntity } = createEntityResponseData ?? {};

        if (!createdEntity) {
          return {
            errors: [
              {
                code: "INVALID_INPUT",
                message: "Error calling createEntity",
              },
            ],
          };
        }

        return {
          data: createdEntity,
        };
      },
      [createFn, ownedById, readonly],
    );

  return {
    createEntity,
  };
};
