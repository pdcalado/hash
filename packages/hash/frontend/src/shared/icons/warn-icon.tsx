import { SvgIcon, SvgIconProps } from "@mui/material";
import { FunctionComponent } from "react";

export const WarnIcon: FunctionComponent<SvgIconProps> = (props) => {
  return (
    <SvgIcon width="20" height="20" viewBox="0 0 20 20" fill="none" {...props}>
      <path
        d="M10 0C4.45312 0 0 4.49219 0 10C0 15.5469 4.45312 20 10 20C15.5078 20 20 15.5469 20 10C20 4.49219 15.5078 0 10 0ZM10 18.125C5.50781 18.125 1.875 14.4922 1.875 10C1.875 5.54688 5.50781 1.875 10 1.875C14.4531 1.875 18.125 5.54688 18.125 10C18.125 14.4922 14.4531 18.125 10 18.125ZM10 11.875C10.5078 11.875 10.9375 11.4844 10.9375 10.9375V5.9375C10.9375 5.42969 10.5078 5 10 5C9.45312 5 9.0625 5.42969 9.0625 5.9375V10.9375C9.0625 11.4844 9.45312 11.875 10 11.875ZM10 13.2031C9.29688 13.2031 8.75 13.75 8.75 14.4141C8.75 15.0781 9.29688 15.625 10 15.625C10.6641 15.625 11.2109 15.0781 11.2109 14.4141C11.2109 13.75 10.6641 13.2031 10 13.2031Z"
        fill="#E77632"
      />
    </SvgIcon>
  );
};
