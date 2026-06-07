import { Effect, Schema } from 'effect'
import { HttpClient } from '../../__shape'
import { decodeResponse } from '../../lib'
import { CommitmentBaseSchema } from './base'

export const CommitmentListResponseSchema = Schema.Struct({
  items: Schema.Array(CommitmentBaseSchema),
})

export type CommitmentListResponse = Schema.Schema.Type<
  typeof CommitmentListResponseSchema
>

export const getCommitmentList = Effect.gen(function* () {
  const httpClient = yield* HttpClient

  const response = yield* httpClient.request({
    path: '/commitments',
    method: 'GET',
  })

  const data = yield* decodeResponse(response, CommitmentListResponseSchema)

  return data.items
})
