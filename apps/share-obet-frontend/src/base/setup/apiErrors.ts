import { Data } from "effect";

export class NetworkError extends Data.TaggedError(
  "NetworkError",
)<{
  readonly cause: unknown;
}> {}

export class DecodeError extends Data.TaggedError(
  "DecodeError",
)<{
  readonly cause: unknown;
}> {}

export class UnexpectedStatusError extends Data.TaggedError(
  "UnexpectedStatusError",
)<{
  readonly status: number;
  readonly body: unknown;
}> {}

export type ApiError =
  | NetworkError
  | DecodeError
  | UnexpectedStatusError;