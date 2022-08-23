import { ApolloError } from "apollo-server-errors";
import { DbAdapter, DbClient } from "../../../db";
import { Entity, UnresolvedGQLLink } from "../../../model";
import {
  CreateLinkInput,
  MutationCreateLinkArgs,
  ResolverFn,
} from "../../apiTypes.gen";
import { LoggedInGraphQLContext } from "../../context";

export const linkEntities = async (
  client: DbClient,
  linkInput: CreateLinkInput,
  createdByAccountId: string,
) => {
  const { sourceAccountId, sourceEntityId } = linkInput;
  const source = await Entity.getEntityLatestVersion(client, {
    accountId: sourceAccountId,
    entityId: sourceEntityId,
  });

  /** @todo: lock the entity on retrieval */

  if (!source) {
    const msg = `entity with fixed ID ${sourceEntityId} not found in account ${sourceAccountId}`;
    throw new ApolloError(msg, "NOT_FOUND");
  }

  const { destinationAccountId, destinationEntityId } = linkInput;

  const destination = await Entity.getEntityLatestVersion(client, {
    accountId: destinationAccountId,
    entityId: destinationEntityId,
  });

  if (!destination) {
    const msg = `entity with fixed ID ${destinationEntityId} not found in account ${destinationAccountId}`;
    throw new ApolloError(msg, "NOT_FOUND");
  }

  const link = await source.createOutgoingLink(client, {
    createdByAccountId,
    stringifiedPath: linkInput.path,
    index: typeof linkInput.index === "number" ? linkInput.index : undefined,
    destination,
  });

  return link.toUnresolvedGQLLink();
};

export const createLink: ResolverFn<
  Promise<UnresolvedGQLLink>,
  {},
  LoggedInGraphQLContext,
  MutationCreateLinkArgs
> = async (_, { link: linkInput }, { dataSources, user }) => {
  return dataSources.db.transaction((client) =>
    linkEntities(client, linkInput, user.accountId),
  );
};
