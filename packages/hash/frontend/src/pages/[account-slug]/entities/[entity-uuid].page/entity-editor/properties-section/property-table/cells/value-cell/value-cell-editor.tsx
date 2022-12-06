import { types } from "@hashintel/hash-shared/types";
import { Box } from "@mui/material";
import { ReactNode, useState } from "react";
import { isValueEmpty } from "../../../is-value-empty";
import { EditorStatus, EditorType, ValueCellEditorComponent } from "./types";
import { ArrayEditor } from "./value-cell-editor/array-editor";
import { UnsortableRow } from "./value-cell-editor/array-editor/unsortable-row";
import { BooleanEditor } from "./value-cell-editor/boolean-editor";
import { EditorTypePicker } from "./value-cell-editor/editor-type-picker";
import { NumberOrStringEditor } from "./value-cell-editor/number-or-string-editor";

const guessEditorTypeFromValue = (
  value: unknown,
  expectedTypes: string[],
): EditorType => {
  if (
    typeof value === "string" &&
    expectedTypes.includes(types.dataType.text.title)
  ) {
    return "string";
  }

  if (
    typeof value === "boolean" &&
    expectedTypes.includes(types.dataType.boolean.title)
  ) {
    return "boolean";
  }

  if (
    typeof value === "number" &&
    expectedTypes.includes(types.dataType.number.title)
  ) {
    return "number";
  }

  return "pick-editor-type";
};

export const guessEditorTypeFromExpectedType = (type: string): EditorType => {
  if (type === types.dataType.text.title) {
    return "string";
  }

  if (type === types.dataType.boolean.title) {
    return "boolean";
  }

  if (type === types.dataType.number.title) {
    return "number";
  }

  return "string";
};

const TypePickerWrapper = ({
  children,
  openTypePicker,
  canChangeType,
  editorType,
}: {
  canChangeType: boolean;
  openTypePicker: () => void;
  children: ReactNode;
  editorType: EditorType;
}) => {
  return (
    <Box sx={{ mt: "1px" }}>
      {children}
      {canChangeType && (
        <Box
          sx={{
            background: "red",
            color: "white",
            border: "1px solid red",
            width: "fit-content",
            p: 0.5,
            cursor: "pointer",
          }}
          onClick={openTypePicker}
        >
          <b>Change Type</b> (current: {editorType})
        </Box>
      )}
    </Box>
  );
};

export const ValueCellEditor: ValueCellEditorComponent = (props) => {
  const { value: cell } = props;
  const { value, expectedTypes, isArray } = cell.data.property;
  const [editorStatus, setEditorStatus] = useState<EditorStatus>(() => {
    const isEmpty = isValueEmpty(value);

    if (expectedTypes.length > 1 && isEmpty) {
      return "pick-editor-type";
    }

    if (isEmpty) {
      return "edit";
    }

    return "view";
  });

  const [editorType, setEditorType] = useState<EditorType>(() => {
    if (isArray) {
      return "array";
    }

    if (expectedTypes.length > 1) {
      return guessEditorTypeFromValue(value, expectedTypes);
    }

    const type = expectedTypes[0];

    if (type === types.dataType.boolean.title) {
      return "boolean";
    }

    if (type === types.dataType.number.title) {
      return "number";
    }

    return "string";
  });

  if (editorType === "array") {
    return <ArrayEditor {...props} />;
  }

  if (editorStatus === "pick-editor-type") {
    return (
      <EditorTypePicker
        cell={cell}
        type={editorType}
        onTypeChange={(type) => {
          setEditorType(type);
          setEditorStatus("edit");
        }}
      />
    );
  }

  if (editorStatus === "view") {
    const wrapperProps = {
      openTypePicker: () => setEditorStatus("pick-editor-type"),
      canChangeType: expectedTypes.length > 1,
      editorType,
    };

    return (
      <UnsortableRow
        value={value}
        onEdit={() => setEditorStatus("edit")}
        openTypePicker={
          expectedTypes.length > 1
            ? () => setEditorStatus("pick-editor-type")
            : undefined
        }
      />
    );
  }

  switch (editorType) {
    case "boolean":
      return <BooleanEditor {...props} />;
    case "number":
      return <NumberOrStringEditor isNumber {...props} />;
    case "string":
      return <NumberOrStringEditor {...props} />;
  }
};
