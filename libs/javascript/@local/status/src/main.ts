import { StatusCode } from "./status-code";

export { type StatusCode } from "./status-code";

/**
 * Defines a logical status and error model that is suitable for different programming environments, including REST APIs
 * and RPC APIs.
 */
export type Status<D> = {
  code: StatusCode;
  message?: string;
  contents: D;
};
