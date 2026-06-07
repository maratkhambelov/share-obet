import { Effect, Schema } from 'effect'
import { HttpClient } from '../../__shape'
import { decodeResponse } from '../../lib'

export const CreateCommitmentSchema = Schema.Struct({
  verifier_id: Schema.String,
  witness_ids: Schema.Array(Schema.String),
  title: Schema.String,
  description: Schema.String,
  end_date: Schema.String,
})

export type CreateCommitment = Schema.Schema.Type<typeof CreateCommitmentSchema>

export const createCommitment = (request: CreateCommitment) =>
  HttpClient.pipe(
    Effect.flatMap((http) =>
      http.request({
        headers: {
          'Content-Type': 'application/json',
        },
        method: 'POST',
        path: '/commitments',
        body: JSON.stringify(request),
      }),
    ),
    Effect.flatMap((response) =>
      decodeResponse(
        response,
        Schema.Struct({
          //временно - потом получим полную инфу CommitmentDetail
          id: Schema.String,
        }),
      ),
    ),
  )
