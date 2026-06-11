import { QueryClient } from '@tanstack/react-query'


export interface CurrentUser {
  userId: string
  displayName: string
}

export const currentUserQueryKey = ['current-user'] as const

export const getCurrentUser = (
  queryClient: QueryClient,
): CurrentUser | undefined => {
  return queryClient.getQueryData<CurrentUser>(
    currentUserQueryKey,
  )
}