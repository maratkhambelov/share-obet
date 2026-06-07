import { Effect, Schema } from 'effect'
import { CommitmentBaseSchema } from './base.ts'
import { HttpClient } from '@/base/setup'
import { decodeResponse } from '@/base/lib'

export const CommitmentDetailSchema = Schema.extend(
  CommitmentBaseSchema,
  Schema.Struct({
    promisor_id: Schema.String,
    verifier_id: Schema.String,
    witness_ids: Schema.Array(Schema.String),
    description: Schema.String,
    created_at: Schema.String,
  }),
)

export type CommitmentDetail = Schema.Schema.Type<typeof CommitmentDetailSchema>

export const getCommitment = (id: string) =>
  Effect.gen(function* () {
    const httpClient = yield* HttpClient

    const response = yield* httpClient.request({
      path: `/commitments/${id}`,
      method: 'GET',
    })

    const data = yield* decodeResponse(response, CommitmentDetailSchema)

    return data
  })
