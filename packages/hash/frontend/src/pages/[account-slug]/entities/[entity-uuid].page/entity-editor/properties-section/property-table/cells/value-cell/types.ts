import { CustomCell, ProvideEditorComponent } from "@glideapps/glide-data-grid";
import { ComponentProps, FunctionComponent } from "react";
import { TooltipCellProps } from "../../../../../../../../../components/grid/utils/use-grid-tooltip/types";
import { PropertyRow } from "../../types";

export interface ValueCellProps extends TooltipCellProps {
  readonly kind: "value-cell";
  property: PropertyRow;
}

export type ValueCell = CustomCell<ValueCellProps>;

export type EditorType = "array" | "boolean" | "number" | "string";
export type EditorStatus = "view" | "edit" | "pick-editor-type";

export type OnTypeChange = (type: EditorType) => void;

export type ValueCellEditorComponent<ExtraProps extends {} = {}> =
  FunctionComponent<
    ComponentProps<ProvideEditorComponent<ValueCell>> & ExtraProps
  >;
