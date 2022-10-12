import { FunctionComponent, useMemo, useState, useRef, useEffect } from "react";
import { Box, Typography } from "@mui/material";
import {
  Avatar,
  Button,
  FontAwesomeIcon,
  IconButton,
} from "@hashintel/hash-design-system/ui";
import { formatDistanceToNowStrict } from "date-fns";
import {
  faChevronDown,
  faChevronUp,
  faEllipsisVertical,
  IconDefinition,
} from "@fortawesome/free-solid-svg-icons";
import { usePopupState } from "material-ui-popup-state/hooks";
import { bindTrigger } from "material-ui-popup-state";
import { PageComment } from "../../../components/hooks/usePageComments";
import { CommentTextField, CommentTextFieldRef } from "./CommentTextField";
import { CommentBlockMenu } from "./CommentBlockMenu";
import styles from "./style.module.css";

const LINE_HEIGHT = 21;

type ShowMoreTextLinkProps = {
  label: string;
  icon: IconDefinition;
  onClick: () => void;
};

export const ToggleTextExpandedButton: FunctionComponent<
  ShowMoreTextLinkProps
> = ({ label, icon, onClick }) => (
  <Button
    size="xs"
    variant="tertiary_quiet"
    sx={{
      display: "flex",
      fontWeight: 600,
      px: 0.5,
      py: 0,
      minHeight: 0,
      color: ({ palette }) => palette.primary.main,
      alignSelf: "flex-end",
    }}
    onClick={onClick}
  >
    {label}
    <FontAwesomeIcon icon={icon} sx={{ fontSize: 12, ml: 0.75 }} />
  </Button>
);

type CommentProps = {
  comment: PageComment;
};

export const CommentBlock: FunctionComponent<CommentProps> = ({ comment }) => {
  const { hasText, author, textUpdatedAt } = comment;

  const contentRef = useRef<CommentTextFieldRef>(null);
  const [collapsed, setCollapsed] = useState(true);
  const [shouldCollapse, setShouldCollapse] = useState(false);

  const commentCreatedAt = useMemo(() => {
    const updatedAt = new Date(textUpdatedAt);
    const timeDistance = formatDistanceToNowStrict(updatedAt);
    return timeDistance === "0 seconds"
      ? "Just now"
      : `${formatDistanceToNowStrict(updatedAt)} ago`;
  }, [textUpdatedAt]);

  const commentMenuPopupState = usePopupState({
    variant: "popover",
    popupId: "comment-block-menu",
  });

  useEffect(() => {
    if (contentRef.current) {
      const dom = contentRef.current.getDom();
      if (dom) {
        setShouldCollapse(dom.scrollHeight > LINE_HEIGHT * 2);
      }
    }
  }, []);

  // TODO: REPLACE WITH ACTUAL TYPED PROPERTY URI
  const preferredName = (author?.properties as any)[
    "http://localhost:3000/@example/types/property-type/preferred-name/"
  ];

  return (
    <Box display="flex" flexDirection="column" p={2}>
      <Box display="flex" justifyContent="space-between">
        <Avatar size={36} title={preferredName ?? "U"} />
        <Box
          sx={{ flexDirection: "column", flex: 1, overflow: "hidden", pl: 1.5 }}
        >
          <Typography
            component="p"
            variant="microText"
            sx={{
              whiteSpace: "nowrap",
              textOverflow: "ellipsis",
              overflow: "hidden",
              fontWeight: 500,
              color: ({ palette }) => palette.gray[90],
            }}
          >
            {preferredName}
          </Typography>
          <Typography
            component="p"
            variant="microText"
            sx={{
              color: ({ palette }) => palette.gray[70],
            }}
          >
            {commentCreatedAt}
          </Typography>
        </Box>

        <IconButton
          {...bindTrigger(commentMenuPopupState)}
          sx={{
            color: ({ palette }) => palette.gray[40],
          }}
        >
          <FontAwesomeIcon icon={faEllipsisVertical} />
        </IconButton>
      </Box>

      <Box p={0.5} pt={2} position="relative">
        <CommentTextField
          ref={contentRef}
          initialText={hasText}
          classNames={collapsed ? styles.Comment__TextField_collapsed! : ""}
          readOnly
        />
        {shouldCollapse && collapsed ? (
          <Box
            sx={{
              display: "flex",
              flexDirection: "row",
              position: "absolute",
              bottom: 4,
              right: 0,
              background:
                "linear-gradient(90deg, transparent 0, rgba(255,255,255,1) 16px, rgba(255,255,255,1) 100%)",
            }}
          >
            <Typography
              sx={{
                fontSize: 14,
                lineHeight: "150%",
                paddingRight: 1,
                paddingLeft: 2,
              }}
            >
              ...
            </Typography>
            <ToggleTextExpandedButton
              label="Show More"
              icon={faChevronDown}
              onClick={() => setCollapsed(false)}
            />
          </Box>
        ) : null}
      </Box>

      {shouldCollapse && !collapsed ? (
        <ToggleTextExpandedButton
          label="Show Less"
          icon={faChevronUp}
          onClick={() => setCollapsed(true)}
        />
      ) : null}

      <CommentBlockMenu popupState={commentMenuPopupState} />
    </Box>
  );
};
