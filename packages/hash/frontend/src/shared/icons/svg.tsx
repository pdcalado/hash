import dynamic from "next/dynamic";

export { default as DropdownIcon } from "./svg/arrow-down-drop.svg";
export { default as KeyboardReturnIcon } from "./svg/keyboard-return.svg";
export { default as LogoIcon } from "./svg/logo.svg";

export type CustomIconName = keyof typeof CUSTOM_ICONS;

export const CUSTOM_ICONS = {
  "circle-plus": dynamic(() => import("./svg/circle-plus.svg")),
  "human-greeting": dynamic(() => import("./svg/human-greeting.svg")),
  info: dynamic(() => import("./svg/info.svg")),
  people: dynamic(() => import("./svg/people.svg")),
  spinner: dynamic(() => import("./svg/spinner.svg")),
  links: dynamic(() => import("./svg/links.svg")),
  shapes: dynamic(() => import("./svg/shapes.svg")),
  "pencil-simple": dynamic(() => import("./svg/pencil-simple-line.svg")),
};
