import { ProsemirrorManager } from "@hashintel/hash-shared/ProsemirrorManager";
import { useRouter } from "next/router";
import { Schema } from "prosemirror-model";
import { EditorView } from "prosemirror-view";
import "prosemirror-view/style/prosemirror.css";
import { useLayoutEffect, useRef, FunctionComponent } from "react";
import { useLocalstorageState } from "rooks";

import { Button } from "@hashintel/hash-design-system";
import Box from "@mui/material/Box";
import { GlobalStyles } from "@mui/material";
import { BlockLoadedProvider } from "../onBlockLoaded";
import { UserBlocksProvider } from "../userBlocks";
import { EditorConnection } from "./collab/EditorConnection";
import { BlocksMap, createEditorView } from "./createEditorView";
import { usePortals } from "./usePortals";
import { useReadonlyMode } from "../../shared/readonly-mode";
import { usePageContext } from "./PageContext";
import { CommentThread } from "./Comments/CommentThread";
import { PageThread } from "../../components/hooks/usePageComments";
import { useCreateComment } from "../../components/hooks/useCreateComment";
import {
  PAGE_CONTENT_WIDTH,
  PAGE_MIN_PADDING,
} from "../../pages/[account-slug]/[page-slug].page";
import { useInitTypeSystem } from "../../lib/use-init-type-system";

type PageBlockProps = {
  blocks: BlocksMap;
  pageComments: PageThread[];
  accountId: string;
  entityId: string;
  containerPadding: [string, string];
};

/**
 * The naming of this as a "Block" is… interesting, considering it doesn't
 * really work like a Block. It would be cool to somehow detach the process of
 * rendering child blocks from this and have a renderer, but it seems tricky to
 * do that
 */
export const PageBlock: FunctionComponent<PageBlockProps> = ({
  blocks,
  pageComments,
  accountId,
  entityId,
  containerPadding,
}) => {
  const loadingTypeSystem = useInitTypeSystem();
  const root = useRef<HTMLDivElement>(null);
  const [portals, renderPortal, clearPortals] = usePortals();
  const [debugging] = useLocalstorageState<
    { restartCollabButton?: boolean } | boolean
  >("hash.internal.debugging", false);

  const prosemirrorSetup = useRef<null | {
    view: EditorView<Schema>;
    connection: EditorConnection | null;
    manager: ProsemirrorManager;
  }>(null);

  const router = useRouter();
  const routeHash = router.asPath.split("#")[1] ?? "";
  const { readonlyMode } = useReadonlyMode();

  const [createComment, { loading: createCommentLoading }] =
    useCreateComment(entityId);

  const { setEditorView, pageTitleRef } = usePageContext();

  /**
   * This effect runs once and just sets up the prosemirror instance. It is not
   * responsible for setting the contents of the prosemirror document
   */
  useLayoutEffect(() => {
    const node = root.current!;

    /**
     * Lets see up prosemirror with an empty document, as another effect will
     * set its contents. Unfortunately all prosemirror documents have to
     * contain at least one child, so lets insert a special "blank" placeholder
     * child
     */
    const { view, connection, manager } = createEditorView(
      node,
      renderPortal,
      accountId,
      entityId,
      blocks,
      readonlyMode,
      pageTitleRef,
    );

    setEditorView(view);

    prosemirrorSetup.current = {
      view,
      connection: connection ?? null,
      manager,
    };

    return () => {
      clearPortals();
      view.destroy();
      connection.close();
      prosemirrorSetup.current = null;
    };
  }, [
    accountId,
    blocks,
    entityId,
    renderPortal,
    readonlyMode,
    clearPortals,
    setEditorView,
    pageTitleRef,
  ]);

  return (
    <UserBlocksProvider value={blocks}>
      <BlockLoadedProvider routeHash={routeHash}>
        <GlobalStyles
          styles={{
            /**
             * to handle margin-clicking, prosemirror should take full width, and give padding to it's content
             * so it automatically handles focusing on closest node on margin-clicking
             */
            ".ProseMirror": {
              padding: `0 ${containerPadding[1]} 320px ${containerPadding[0]}`,
              minWidth: `calc(${PAGE_CONTENT_WIDTH}px + (${PAGE_MIN_PADDING}px * 2))`,
            },
            // prevents blue outline on selected nodes
            ".ProseMirror-selectednode": { outline: "none" },
          }}
        />
        <Box id="root" ref={root} position="relative">
          {loadingTypeSystem ? null : (
            <Box
              sx={{
                position: "absolute",
                top: 0,
                right: containerPadding[1],
                transform: "translateX(calc(100% + 48px))",
                zIndex: 1,
              }}
            >
              {pageComments?.map((comment) => (
                <CommentThread
                  key={comment.entityId}
                  comment={comment}
                  createComment={createComment}
                  loading={createCommentLoading}
                />
              ))}
            </Box>
          )}
        </Box>
        {portals}
        {/**
         * @todo position this better
         */}
        {(
          typeof debugging === "boolean"
            ? debugging
            : debugging.restartCollabButton
        ) ? (
          <Button
            sx={{
              position: "fixed",
              bottom: 2.5,
              right: 2.5,
              opacity: 0.3,

              "&:hover": {
                opacity: 1,
              },
            }}
            onClick={() => {
              prosemirrorSetup.current?.connection?.restart();
            }}
          >
            Restart Collab Instance
          </Button>
        ) : null}
      </BlockLoadedProvider>
    </UserBlocksProvider>
  );
};
