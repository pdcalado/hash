import { BlockGraph, Entity } from "@blockprotocol/graph";

export const getLinkedEntities = <T extends Entity = Entity>({
  sourceEntityId,
  blockGraph,
  path,
}: {
  sourceEntityId: string;
  blockGraph: BlockGraph;
  path: string;
}): T[] | null => {
  const { linkGroups, linkedEntities } = blockGraph;
  const matchingLinkGroup = linkGroups.find(
    (linkGroup) =>
      linkGroup.sourceEntityId === sourceEntityId && linkGroup.path === path,
  );
  if (!matchingLinkGroup) {
    return null;
  }
  const destinationEntityIds = matchingLinkGroup.links.map(
    (link) => link.destinationEntityId,
  );
  return linkedEntities.filter((linkedEntity) =>
    destinationEntityIds.includes(linkedEntity.entityId),
  ) as T[];
};

export const mustGetLinkedEntities = (
  ...params: Parameters<typeof getLinkedEntities>
) => {
  const linkedEntities = getLinkedEntities(...params);
  if (!linkedEntities || linkedEntities.length < 1) {
    throw new Error(
      `No entities linked on ${params[0].path} for source entity ${params[0].sourceEntityId}`,
    );
  }
  return linkedEntities;
};
