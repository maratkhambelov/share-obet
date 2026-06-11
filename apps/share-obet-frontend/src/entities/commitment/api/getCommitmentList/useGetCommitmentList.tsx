import { useQuery } from '@tanstack/react-query'
import { commitmentKeys, getCommitmentList } from '@/base/domains/Commitment'
import { runEffect } from '@/base/lib'

export const useCommitmentsQuery = (initData: string) =>
  useQuery({
    queryKey: commitmentKeys.all,
    queryFn: () => runEffect(getCommitmentList(initData)),
  })
