import { DecodeError } from '../setup'
import { Effect, Schema } from 'effect'

export const decodeSchema = <A, I /* ifContextShouldAdd, R */>(
  schema: Schema.Schema<A, I>,
  input: unknown,
) =>
  Schema.decodeUnknown(schema)(input).pipe(
    Effect.mapError(
      (cause) =>
        new DecodeError({
          cause,
        }),
    ),
  )
