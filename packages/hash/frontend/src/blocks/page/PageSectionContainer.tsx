import { Box, SxProps } from "@mui/material";
import { ReactNode } from "react";
import { PageThread } from "../../components/hooks/usePageComments";

export const PAGE_CONTENT_WIDTH = 696;
export const PAGE_MIN_PADDING = 48;
export const COMMENTS_WIDTH = 320;

export const getPageSectionContainerStyles = (pageComments: PageThread[]) => {
  const commentsContainerWidth = pageComments?.length ? COMMENTS_WIDTH : 0;

  const paddingLeft = `max(calc((100% - ${
    PAGE_CONTENT_WIDTH + commentsContainerWidth + PAGE_MIN_PADDING
  }px) / 2), ${PAGE_MIN_PADDING}px)`;
  const paddingRight = `calc((100% - ${PAGE_CONTENT_WIDTH}px - ${paddingLeft}))`;

  return {
    padding: `${PAGE_MIN_PADDING}px ${paddingRight} 0 ${paddingLeft}`,
    minWidth: `calc(${PAGE_CONTENT_WIDTH}px + (${PAGE_MIN_PADDING}px * 2))`,
  };
};

export const PageSectionContainer = ({
  children,
  pageComments,
  sx = [],
}: {
  children: ReactNode;
  pageComments: PageThread[];
  sx?: SxProps;
}) => {
  return (
    <Box
      sx={[
        getPageSectionContainerStyles(pageComments),
        ...(Array.isArray(sx) ? sx : [sx]),
      ]}
    >
      {children}
    </Box>
  );
};
