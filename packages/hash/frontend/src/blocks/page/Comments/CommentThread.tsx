import { FunctionComponent, useState, useRef, useMemo } from "react";
import { TextToken } from "@hashintel/hash-shared/graphql/types";
import { Box, buttonClasses, Collapse } from "@mui/material";
import { Button } from "@hashintel/hash-design-system";
import ExpandLessIcon from "@mui/icons-material/ExpandLess";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import { PageThread } from "../../../components/hooks/usePageComments";
import { CommentTextField, CommentTextFieldRef } from "./CommentTextField";
import { CommentBlock } from "./CommentBlock";
import styles from "../style.module.css";

type CommentThreadProps = {
  comment: PageThread;
  createComment: (parentId: string, content: TextToken[]) => Promise<void>;
  loading: boolean;
};

export const CommentThread: FunctionComponent<CommentThreadProps> = ({
  comment,
  createComment,
  loading,
}) => {
  const inputRef = useRef<CommentTextFieldRef>(null);
  const [threadFocused, setThreadFocused] = useState(false);
  const [inputFocused, setInputFocused] = useState(false);
  const [expanded, setExpanded] = useState(false);
  const [inputValue, setInputValue] = useState<TextToken[]>([]);

  const showInput = threadFocused || !!inputValue.length;
  const showInputButtons =
    (threadFocused && inputFocused) || !!inputValue.length;

  const submitComment = async () => {
    if (!loading && inputValue?.length) {
      await createComment(comment.entityId, inputValue).then(() => {
        inputRef.current?.resetDocument();
      });
    }
  };

  const [collapsedReplies, lastReply] = useMemo(() => {
    const replies = [...comment.replies];
    const lastItem = replies.pop();
    return [replies, lastItem];
  }, [comment]);

  return (
    <Box
      tabIndex={0}
      onFocus={() => setThreadFocused(true)}
      onBlur={() => setThreadFocused(false)}
      sx={({ palette, boxShadows }) => ({
        width: 320,
        background: palette.white,
        borderRadius: 1.5,
        boxShadow: boxShadows.md,
        marginBottom: 4,
      })}
    >
      <CommentBlock key={comment.entityId} comment={comment} />

      {collapsedReplies.length ? (
        <>
          <Button
            variant="tertiary"
            onClick={() => setExpanded(!expanded)}
            size="small"
            sx={({ palette }) => ({
              minHeight: 0,
              height: 40,
              width: 1,
              borderRadius: 0,
              border: "none",
              borderTop: `1px solid ${palette.gray[20]}`,
              [`.${buttonClasses.endIcon}`]: {
                ml: 0.5,
                color: palette.gray[70],
                fontSize: 20,
              },
            })}
            endIcon={expanded ? <ExpandLessIcon /> : <ExpandMoreIcon />}
          >
            {expanded
              ? "Show fewer responses"
              : `Show all ${comment.replies.length} responses`}
          </Button>
          <Collapse in={expanded}>
            {collapsedReplies.map((reply) => (
              <Box
                key={reply.entityId}
                sx={{
                  borderTop: ({ palette }) => `1px solid ${palette.gray[20]}`,
                }}
              >
                <CommentBlock comment={reply} />
              </Box>
            ))}
          </Collapse>
        </>
      ) : null}

      {lastReply ? (
        <Box
          key={lastReply.entityId}
          sx={{
            borderTop: ({ palette }) => `1px solid ${palette.gray[20]}`,
          }}
        >
          <CommentBlock comment={lastReply} />
        </Box>
      ) : null}

      <Collapse in={showInput}>
        <Box
          sx={{
            borderTop: ({ palette }) =>
              comment.replies.length ? `1px solid ${palette.gray[20]}` : "none",
            px: 1,
            pt: comment.replies.length ? 1 : 0,
            pb: 0.75,
          }}
        >
          <Box
            sx={({ palette, transitions }) => ({
              border: `1px solid ${palette.gray[30]}`,
              borderRadius: 1.5,
              pl: 2,
              transition: transitions.create("border-color"),
              "&:focus-within": {
                borderColor: palette.blue[60],
              },
            })}
          >
            <CommentTextField
              ref={inputRef}
              placeholder={`Reply to ${comment.author.properties.preferredName}`}
              onSubmit={submitComment}
              editable={!loading}
              onFocusChange={setInputFocused}
              onChange={setInputValue}
              classNames={styles.Comment__TextField_editable}
            />
          </Box>
        </Box>
      </Collapse>

      <Collapse in={showInputButtons}>
        <Box
          sx={{
            display: "flex",
            gap: 0.75,
            justifyContent: "flex-end",
            px: 1,
            pt: 0,
            pb: 0.75,
          }}
        >
          <Button
            size="xs"
            variant="tertiary"
            onClick={() => {
              inputRef.current?.resetDocument();
            }}
          >
            Cancel
          </Button>
          <Button
            size="xs"
            variant="secondary"
            disabled={!inputValue.length}
            onClick={submitComment}
            loading={loading}
          >
            Reply
          </Button>
        </Box>
      </Collapse>

      <Collapse in={showInput}>
        <Box pb={0.25} />
      </Collapse>
    </Box>
  );
};
