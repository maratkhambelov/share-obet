import { Context, Effect } from 'effect'
import type { ApiError } from './apiErrors.ts'
import type { ApiConfig } from './apiConfig.ts'

export interface HttpClientShape {
  readonly request: (
    request: RequestInit & {
      readonly path: string;
    },
  ) => Effect.Effect<
    Response,
    ApiError,
    ApiConfig
  >;
}

export class HttpClient extends Context.Tag(
  "HttpClient",
)<HttpClient, HttpClientShape>() {}
