import { IconDefinition } from "@fortawesome/free-solid-svg-icons";
import {
  defaultIconStyle,
  FontAwesomeIcon,
} from "@hashintel/hash-design-system";
import { BoxProps, experimental_sx, styled, SvgIconProps } from "@mui/material";
import { forwardRef, SVGProps } from "react";
import { CustomIconName, CUSTOM_ICONS } from "./svg";

type CustomIconProps = {
  icon: CustomIconName;
  sx?: BoxProps["sx"];
} & SVGProps<SVGSVGElement>;

type FontAwesomeIconProps = {
  icon: Pick<IconDefinition, "icon">;
} & SvgIconProps;

export const Icon = forwardRef<
  SVGSVGElement,
  CustomIconProps | FontAwesomeIconProps
>((props, ref) => {
  if (typeof props.icon === "string") {
    const { sx = [], icon, ...rest } = props;
    const IconComponent = CUSTOM_ICONS?.[icon ?? "Info"] || CUSTOM_ICONS.info;

    const SIconComponent = styled(IconComponent)(
      experimental_sx([defaultIconStyle, ...(Array.isArray(sx) ? sx : [sx])]),
    );

    return <SIconComponent ref={ref} {...rest} />;
  }

  return <FontAwesomeIcon ref={ref} {...props} />;
});
