import {  Effect, Layer } from 'effect'

import {
  DecodeError,
  NetworkError,
  UnexpectedStatusError,
  HttpClient
} from '../__shape'
import { ApiConfig } from './apiConfig'

export const HttpClientLive = Layer.succeed(
  HttpClient,
  {
    request: ({ path, ...init }) =>
      Effect.gen(function* () {
        const config =
          yield* ApiConfig;

        const response =
          yield* Effect.tryPromise({
            try: () =>
              fetch(
                `${config.apiUrl}${path}`,
                init,
              ),

            catch: (cause) =>
              new NetworkError({
                cause,
              }),
          });

        if (!response.ok) {
          const body =
            yield* Effect.tryPromise({
              try: () =>
                response.text(),

              catch: (cause) =>
                new DecodeError({
                  cause,
                }),
            });

          return yield* Effect.fail(
            new UnexpectedStatusError({
              status:
              response.status,
              body,
            }),
          );
        }

        return response;
      }),
  },
);