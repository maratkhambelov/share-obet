import { useMutation } from '@tanstack/react-query'
import {
  createCommitment,
  type CreateCommitment,
} from '@/base/domains/Commitment'
import { runEffect } from '@/base/lib'

export const useCreateCommitmentMutation = () => {
  return useMutation({
    mutationFn: (request: CreateCommitment) =>
      runEffect(createCommitment(request)),
  })
}
