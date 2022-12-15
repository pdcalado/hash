import { ErrorCode } from "./error-code";

export { type ErrorCode } from "./error-code";

/**
 * Defines a logical error model that is suitable for different programming environments, including REST APIs and RPC
 * APIs.
 */
export type Status<D> = {
  code: ErrorCode;
  message?: string;
  contents: D;
};
