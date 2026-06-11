import { Button } from '@/shared/ui/button'
import { useCommitmentsQuery } from '@/entities/commitment'

import {
  Card,
  CardHeader,
  CardTitle,
} from '@/shared/ui/card'
import { useRawInitData } from '@tma.js/sdk-react'

type CommitmentListPageProps = {
  readonly onCreate: () => void
  readonly onOpen: (
    commitmentId: string,
  ) => void
}

export const CommitmentListPage = ({
                                     onCreate,
                                     onOpen,
                                   }: CommitmentListPageProps) => {

  const initData = useRawInitData() ?? ''

  const { data: commitments = [] } =
    useCommitmentsQuery(initData)

  return (
    <div className="space-y-6">
      <Button onClick={onCreate}>
        Создать обязательство
      </Button>

      <h1>Commitments</h1>

      <div className="space-y-4">
        {commitments.map((commitment) => (
          <Card key={commitment.id}>
            <CardHeader>
              <CardTitle
                onClick={() =>
                  onOpen(commitment.id)
                }
              >
                {commitment.title}
              </CardTitle>
            </CardHeader>
          </Card>
        ))}
      </div>
    </div>
  )
}