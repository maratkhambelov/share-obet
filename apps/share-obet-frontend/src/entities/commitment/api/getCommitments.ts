import { Effect, Schema } from 'effect'
import { HttpClient } from '@/base/setup'
import { decodeResponse } from '@/base/lib'
import { CommitmentBaseSchema } from './base'

export const CommitmentListResponseSchema = Schema.Struct({
  items: Schema.Array(CommitmentBaseSchema),
})

export type CommitmentListResponse = Schema.Schema.Type<
  typeof CommitmentListResponseSchema
>

export const getCommitments = Effect.gen(function* () {
  const httpClient = yield* HttpClient

  const response = yield* httpClient.request({
    path: '/commitments',
    method: 'GET',
  })

  const data = yield* decodeResponse(response, CommitmentListResponseSchema)

  return data.items
})
