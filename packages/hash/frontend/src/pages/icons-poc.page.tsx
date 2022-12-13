import { faCalendarCheck } from "@fortawesome/free-solid-svg-icons";
import { Box } from "@mui/material";
import { Icon } from "../shared/icons/icon";

const Page = () => {
  return (
    <Box
      sx={{
        p: 5,
        display: "flex",
        flexDirection: "column",

        svg: {
          border: "1px solid red !important",
          width: 32,
          height: 32,
        },
      }}
    >
      <Icon icon="circle-plus" />
      <Icon icon="human-greeting" />
      <Icon icon="info" />
      <Icon icon="people" />
      <Icon icon="spinner" />
      <Icon icon="links" />
      <Icon icon="shapes" />
      <Icon icon="pencil-simple" />
      <Icon icon={faCalendarCheck} />
    </Box>
  );
};

export default Page;
