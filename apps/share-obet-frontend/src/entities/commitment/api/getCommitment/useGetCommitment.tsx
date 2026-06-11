import { skipToken, useQuery } from '@tanstack/react-query'
import { commitmentKeys, getCommitment } from '@/base/domains/Commitment'
import { runEffect } from '@/base/lib'

// export const useCommitmentQuery = (
//   id: string | null,
// ) =>
//   useQuery({
//     queryKey: commitmentKeys.detail(id ?? ''),
//     queryFn: () => runEffect(getCommitment(id!)),
//     enabled: Boolean(id),
//   })

export const useCommitmentQuery = (id: string | null, initData: string) => {

  const QUERY_PARAMS = id
    ? {
        queryKey: commitmentKeys.detail(id),
        queryFn: () => runEffect(getCommitment(id, initData)),
      } as const
    : {
        queryKey: commitmentKeys.detail('__skipToken__'),
        queryFn: skipToken,
      } as const

  return useQuery({
    ...QUERY_PARAMS,
  })
}
