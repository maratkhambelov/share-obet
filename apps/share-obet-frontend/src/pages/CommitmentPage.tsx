import { useCallback, useState } from 'react'

import {
  useCommitmentQuery,
  useCreateCommitmentMutation,
} from '@/entities/commitment'

import { Button } from '@/shared/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/shared/ui/card'
import { Input } from '@/shared/ui/input'
import { Textarea } from '@/shared/ui/textarea'
import { useRawInitData } from '@tma.js/sdk-react'

type CommitmentPageProps = {
  readonly commitmentId?: string
  readonly onBack: () => void
}

export const CommitmentPage = ({
  commitmentId,
  onBack,
}: CommitmentPageProps) => {

  const initData = useRawInitData() ?? ''

  const { data: commitment, isLoading } = useCommitmentQuery(
    commitmentId ?? null,
    initData
  )

  const createCommitment = useCreateCommitmentMutation(initData)
  const [title, setTitle] = useState('')
  const [description, setDescription] = useState('')

  const handleCreate = useCallback(() => {
    const TEMP_VERIFIER_ID = '550e8400-e29b-41d4-a716-446655440000'
    const TEMP_WITNESS_IDS = [
      '550e8400-e29b-41d4-a716-446655440001',
      '550e8400-e29b-41d4-a716-446655440002',
    ]

    createCommitment.mutateAsync({
      verifier_id: TEMP_VERIFIER_ID,
      witness_ids: TEMP_WITNESS_IDS,
      title,
      description,
      end_date: new Date().toISOString(),
    }).then(() => {
      onBack()
    })
  }, [createCommitment, description, onBack, title])

  if (isLoading) {
    return (
      <div className="mx-auto max-w-2xl p-6">
        <p className="mt-4">Loading...</p>
      </div>
    )
  }


  if (!commitmentId || !commitment) {
    return (
      <div className="mx-auto max-w-2xl space-y-6 p-6">
        <Button variant="outline" onClick={onBack}>
          Назад
        </Button>
        <Card>
          <CardHeader>
            <CardTitle>Создать обязательство</CardTitle>
          </CardHeader>

          <CardContent className="space-y-4">
            <div className="space-y-2">
              <label>Название</label>

              <Input
                value={title}
                onChange={(event) => setTitle(event.target.value)}
                placeholder="Похудеть до 80 кг"
              />
            </div>

            <div className="space-y-2">
              <label>Описание</label>

              <Textarea
                value={description}
                onChange={(event) => setDescription(event.target.value)}
                placeholder="Описание обязательства"
              />
            </div>

            <Button
              onClick={handleCreate}
              disabled={createCommitment.isPending}
            >
              {createCommitment.isPending ? 'Создание...' : 'Создать'}
            </Button>
          </CardContent>
        </Card>
      </div>
    )
  }


  // if (!commitment) {
  //   return (
  //     <div className="mx-auto max-w-2xl p-6">
  //       <Button variant="outline" onClick={onBack}>
  //         Назад
  //       </Button>
  //       <p className="mt-4">Commitment not found</p>
  //     </div>
  //   )
  // }

  return (
    <div className="mx-auto max-w-2xl space-y-6 p-6">
      <Button variant="outline" onClick={onBack}>
        Назад
      </Button>
      <Card>
        <CardHeader>
          <CardTitle>{commitment.title}</CardTitle>
        </CardHeader>

        <CardContent>
          <p>{commitment.description}</p>
        </CardContent>
      </Card>
    </div>
  )
}
