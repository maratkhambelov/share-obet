import { Schema } from 'effect'


export const CommitmentBaseSchema =
  Schema.Struct({
    id: Schema.String,
    title: Schema.String,
    status: Schema.String,
    end_date: Schema.String,
  })

export type CommitmentBase =
  Schema.Schema.Type<
    typeof CommitmentBaseSchema
  >