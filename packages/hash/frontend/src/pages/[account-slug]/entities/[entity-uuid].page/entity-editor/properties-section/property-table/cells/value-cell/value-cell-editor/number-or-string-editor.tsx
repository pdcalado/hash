import { TextField } from "@hashintel/hash-design-system";
import produce from "immer";
import { ValueCellEditorComponent } from "../types";

export const NumberOrStringEditor: ValueCellEditorComponent<{
  isNumber?: boolean;
}> = ({ value: cell, onChange, isNumber }) => {
  const { value } = cell.data.property;

  return (
    <TextField
      sx={{ width: "100%" }}
      autoFocus
      value={value}
      type={isNumber ? "number" : "text"}
      inputMode={isNumber ? "numeric" : "text"}
      onChange={({ target }) => {
        const newCell = produce(cell, (draftCell) => {
          const isEmptyString = target.value === "";

          const newValue =
            isNumber && !isEmptyString ? Number(target.value) : target.value;

          draftCell.data.property.value = newValue;
        });

        onChange(newCell);
      }}
    />
  );
};
