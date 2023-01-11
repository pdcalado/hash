import { useMutation } from "@apollo/client";
import { EmbedderGraphMessageCallbacks } from "@blockprotocol/graph";
import { EntityId } from "@hashintel/hash-shared/types";
import { useCallback } from "react";

import {
  ArchiveEntityMutation,
  ArchiveEntityMutationVariables,
} from "../../../../graphql/api-types.gen";
import { archiveEntityMutation } from "../../../../graphql/queries/knowledge/entity.queries";

export const useBlockProtocolArchiveEntity = (
  readonly?: boolean,
): {
  archiveEntity: EmbedderGraphMessageCallbacks["deleteEntity"];
} => {
  const [archiveEntityFn] = useMutation<
    ArchiveEntityMutation,
    ArchiveEntityMutationVariables
  >(archiveEntityMutation, {
    /** @todo reconsider caching. This is done for testing/demo purposes. */
    fetchPolicy: "no-cache",
  });

  const archiveEntity: EmbedderGraphMessageCallbacks["deleteEntity"] =
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

        if (!data) {
          return {
            errors: [
              {
                code: "INVALID_INPUT",
                message: "'data' must be provided for archiveEntity",
              },
            ],
          };
        }

        const { entityId } = data;

        await archiveEntityFn({
          variables: { entityId: entityId as EntityId }, // @todo-0.3 consider validating that this matches the id format
        });

        return { data: true };
      },
      [archiveEntityFn, readonly],
    );

  return { archiveEntity };
};
