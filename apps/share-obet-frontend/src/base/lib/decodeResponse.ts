import { decodeSchema } from '@/base/lib/decodeSchema.ts'
import { Effect, Schema } from 'effect'
import { parseJson } from '@/base/lib/parseJson.ts'

export const decodeResponse = <A, I>(
  response: Response,
  schema: Schema.Schema<A, I>,
) =>
  parseJson(response).pipe(Effect.flatMap((json) => decodeSchema(schema, json)))
