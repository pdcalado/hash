import { Components, Theme } from "@mui/material";
import { MuiChipThemeOptions } from "./components/data-display/mui-chip-theme-options";
import {
  MuiIconButtonThemeOptions,
  MuiListItemButtonThemeOptions,
  MuiListItemTextThemeOptions,
  MuiTooltipThemeOptions,
  MuiListItemSecondaryActionThemeOptions,
} from "./components/data-display";
import {
  MuiDrawerThemeOptions,
  MuiMenuThemeOptions,
  MuiMenuItemThemeOptions,
} from "./components/navigation";
import {
  MuiButtonThemeOptions,
  MuiOutlinedInputThemeOptions,
  MuiRadioThemeOptions,
  MuiCheckboxThemeOptions,
  MuiInputLabelThemeOptions,
  MuiSwitchThemeOptions,
  MuiFormHelperTextThemeOptions,
} from "./components/inputs";
import { MuiCssBaselineThemeOptions } from "./components/utils";
import { MuiSelectThemeOptions } from "./components/inputs/mui-select-theme-options";
import { MuiSkeletonThemeOptions } from "./components/feedback/mui-skeleton-theme-options";

export const components: Components<Theme> = {
  /** ===== INPUTS ===== */
  MuiButton: MuiButtonThemeOptions,
  MuiOutlinedInput: MuiOutlinedInputThemeOptions,
  MuiInputLabel: MuiInputLabelThemeOptions,
  MuiSwitch: MuiSwitchThemeOptions,
  MuiRadio: MuiRadioThemeOptions,
  MuiCheckbox: MuiCheckboxThemeOptions,
  MuiSelect: MuiSelectThemeOptions,
  MuiFormHelperText: MuiFormHelperTextThemeOptions,
  /** ===== DATA DISPLAY ===== */
  MuiChip: MuiChipThemeOptions,
  MuiIconButton: MuiIconButtonThemeOptions,
  MuiTooltip: MuiTooltipThemeOptions,
  MuiListItemButton: MuiListItemButtonThemeOptions,
  MuiListItemText: MuiListItemTextThemeOptions,
  MuiListItemSecondaryAction: MuiListItemSecondaryActionThemeOptions,

  /** ===== FEEDBACK ===== */
  MuiSkeleton: MuiSkeletonThemeOptions,

  /** ===== SURFACES ===== */
  /** ===== NAVIGATION ===== */
  MuiDrawer: MuiDrawerThemeOptions,
  MuiMenu: MuiMenuThemeOptions,
  MuiMenuItem: MuiMenuItemThemeOptions,

  /** ===== LAYOUT ===== */
  /** ===== UTILS ===== */
  MuiCssBaseline: MuiCssBaselineThemeOptions,
  /** ===== DATA GRID ===== */
};
