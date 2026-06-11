import { Effect, Schema } from 'effect'

import { HttpClient } from '../__shape'
import { decodeResponse } from '../lib'

export const AuthenticateTelegramRequestSchema = Schema.Struct({
  init_data: Schema.String,
})

export type AuthenticateTelegramRequest = Schema.Schema.Type<
  typeof AuthenticateTelegramRequestSchema
>

export const AuthenticateTelegramResponseSchema = Schema.Struct({
  user_id: Schema.String,
  display_name: Schema.String,
})

export const authenticateTelegram = (request: AuthenticateTelegramRequest) => {
  const effect = HttpClient.pipe(
    Effect.flatMap((http) =>
      http.request({
        headers: {
          'Content-Type': 'application/json',
        },
        method: 'POST',
        path: '/auth/telegram',
        body: JSON.stringify(request),
      }),
    ),
    Effect.flatMap((response) =>
      decodeResponse(response, AuthenticateTelegramResponseSchema),
    ),
  )

  return effect
}
