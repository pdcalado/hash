import { CommentModel } from "../../../../model";
import { ResolverFn } from "../../../apiTypes.gen";
import { LoggedInGraphQLContext } from "../../../context";
import { UnresolvedPersistedCommentGQL } from "../model-mapping";

export const persistedCommentTextUpdatedAt: ResolverFn<
  Promise<string>,
  UnresolvedPersistedCommentGQL,
  LoggedInGraphQLContext,
  {}
> = async ({ entityId }, _, { dataSources: { graphApi } }) => {
  const comment = await CommentModel.getCommentById(graphApi, { entityId });
  const textEntity = await comment.getHasText(graphApi);

  return textEntity.version;
};
