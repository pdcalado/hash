/**
 * This file contains the Block Protocol callbacks that HASH provides to blocks
 * using the GraphQL API.
 *
 * The intention is that the contents of this file will replace the existing set
 * of functions provided to blocks, but for now the added operations are mostly
 * relevant for the type editors.
 */
import { EmbedderGraphMessageCallbacks } from "@blockprotocol/graph";
import { OwnedById } from "@hashintel/hash-shared/types";

import { KnowledgeCallbacks } from "../../components/hooks/block-protocol-functions/knowledge/knowledge-shim";
import { useBlockProtocolAggregateEntities } from "../../components/hooks/block-protocol-functions/knowledge/use-block-protocol-aggregate-entities";
import { useBlockProtocolArchiveEntity } from "../../components/hooks/block-protocol-functions/knowledge/use-block-protocol-archive-entity";
import { useBlockProtocolCreateEntity } from "../../components/hooks/block-protocol-functions/knowledge/use-block-protocol-create-entity";
// Knowledge Graph Operations
import { useBlockProtocolGetEntity } from "../../components/hooks/block-protocol-functions/knowledge/use-block-protocol-get-entity";
import { useBlockProtocolUpdateEntity } from "../../components/hooks/block-protocol-functions/knowledge/use-block-protocol-update-entity";
import { OntologyCallbacks } from "../../components/hooks/block-protocol-functions/ontology/ontology-types-shim";
// Ontology operations
import { useBlockProtocolAggregateDataTypes } from "../../components/hooks/block-protocol-functions/ontology/use-block-protocol-aggregate-data-types";
import { useBlockProtocolAggregateEntityTypes } from "../../components/hooks/block-protocol-functions/ontology/use-block-protocol-aggregate-entity-types";
import { useBlockProtocolAggregatePropertyTypes } from "../../components/hooks/block-protocol-functions/ontology/use-block-protocol-aggregate-property-types";
import { useBlockProtocolCreateEntityType } from "../../components/hooks/block-protocol-functions/ontology/use-block-protocol-create-entity-type";
import { useBlockProtocolCreatePropertyType } from "../../components/hooks/block-protocol-functions/ontology/use-block-protocol-create-property-type";
import { useBlockProtocolGetDataType } from "../../components/hooks/block-protocol-functions/ontology/use-block-protocol-get-data-type";
import { useBlockProtocolGetEntityType } from "../../components/hooks/block-protocol-functions/ontology/use-block-protocol-get-entity-type";
import { useBlockProtocolGetPropertyType } from "../../components/hooks/block-protocol-functions/ontology/use-block-protocol-get-property-type";
import { useBlockProtocolUpdateEntityType } from "../../components/hooks/block-protocol-functions/ontology/use-block-protocol-update-entity-type";
import { useBlockProtocolUpdatePropertyType } from "../../components/hooks/block-protocol-functions/ontology/use-block-protocol-update-property-type";
import { useBlockProtocolFileUpload } from "../../components/hooks/block-protocol-functions/use-block-protocol-file-upload";
import { useIsReadonlyMode } from "../../shared/readonly-mode";

export type GraphMessageCallbacks = Omit<
  EmbedderGraphMessageCallbacks,
  | "getEntity"
  | "createEntity"
  | "aggregateEntities"
  | "updateEntity"
  | "getEntityType"
  | "createLink"
  | "getLink"
  | "updateLink"
  | "deleteLink"
  | "getLinkedAggregation"
  | "deleteEntity"
  | "deleteEntityType"
  | "createLinkedAggregation"
  | "updateLinkedAggregation"
  | "deleteLinkedAggregation"
  // Replaced by new ontology callbacks
  | "createEntityType"
  | "updateEntityType"
  | "aggregateEntityTypes"
> &
  OntologyCallbacks &
  KnowledgeCallbacks;

/** @todo Consider if we should move this out of the page and into the hooks directory. */
export const useBlockProtocolFunctionsWithOntology = (
  ownedById: OwnedById | null,
): GraphMessageCallbacks => {
  const isReadonlyMode = useIsReadonlyMode();

  const { aggregateEntities } = useBlockProtocolAggregateEntities();
  const { createEntity } = useBlockProtocolCreateEntity(
    ownedById,
    isReadonlyMode,
  );

  const { getEntity } = useBlockProtocolGetEntity();
  const { updateEntity } = useBlockProtocolUpdateEntity();
  const { archiveEntity } = useBlockProtocolArchiveEntity();

  const { uploadFile } = useBlockProtocolFileUpload(isReadonlyMode);

  // Ontology operations

  const { aggregateDataTypes } = useBlockProtocolAggregateDataTypes();
  const { getDataType } = useBlockProtocolGetDataType();
  const { createPropertyType } = useBlockProtocolCreatePropertyType(
    ownedById,
    isReadonlyMode,
  );
  const { aggregatePropertyTypes } = useBlockProtocolAggregatePropertyTypes();
  const { getPropertyType } = useBlockProtocolGetPropertyType();
  const { updatePropertyType } =
    useBlockProtocolUpdatePropertyType(isReadonlyMode);
  const { createEntityType } = useBlockProtocolCreateEntityType(
    ownedById,
    isReadonlyMode,
  );
  const { aggregateEntityTypes } = useBlockProtocolAggregateEntityTypes();
  const { getEntityType } = useBlockProtocolGetEntityType();
  const { updateEntityType } = useBlockProtocolUpdateEntityType(isReadonlyMode);

  return {
    aggregateEntities,
    createEntity,
    uploadFile,
    // Knowledge operations
    getEntity,
    updateEntity,
    archiveEntity,
    // Ontology operations
    aggregateDataTypes,
    getDataType,
    createPropertyType,
    aggregatePropertyTypes,
    getPropertyType,
    updatePropertyType,
    createEntityType,
    aggregateEntityTypes,
    getEntityType,
    updateEntityType,
  };
};
