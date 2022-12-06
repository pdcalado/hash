import { faPencil } from "@fortawesome/free-solid-svg-icons";
import { Box } from "@mui/material";
import { ValueChip } from "./value-chip";
import { RowAction } from "./row-action";

interface UnsortableRowProps {
  onEdit: () => void;
  value: unknown;
  openTypePicker?: () => void;
}

export const UnsortableRow = ({
  onEdit,
  value,
  openTypePicker,
}: UnsortableRowProps) => {
  return (
    <Box
      sx={{
        height: 48,
        display: "flex",
        alignItems: "center",
        border: "1px solid",
        borderColor: "gray.20",
        position: "relative",
        outline: "none",
        background: "white",
        px: 1.5,
        borderRadius: (theme) => theme.borderRadii.lg,
      }}
    >
      <ValueChip value={value} selected={false} onTypeClick={openTypePicker} />

      <Box
        display="flex"
        sx={{
          position: "absolute",
          inset: 0,
          left: "unset",
          "::before": {
            content: `""`,
            width: 50,
            background: `linear-gradient(90deg, transparent 0%, white 100%)`,
          },
        }}
      >
        <Box sx={{ display: "flex", background: "white" }}>
          <RowAction tooltip="Edit" icon={faPencil} onClick={onEdit} />
        </Box>
      </Box>
    </Box>
  );
};
