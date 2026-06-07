import { useState } from 'react'

import { useCommitmentQuery } from '@/entities/commitment'

import { Button } from '@/shared/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/shared/ui/card'
import { Input } from '@/shared/ui/input'
import { Textarea } from '@/shared/ui/textarea'

type CommitmentPageProps = {
  readonly commitmentId?: string
  readonly onBack: () => void
}

export const CommitmentPage = ({
  commitmentId,
  onBack,
}: CommitmentPageProps) => {
  const isCreateMode = commitmentId === undefined

  const { data: commitment, isLoading } = useCommitmentQuery(commitmentId)

  const [title, setTitle] = useState('')
  const [description, setDescription] = useState('')

  if (isCreateMode) {
    return (
      <div className="mx-auto max-w-2xl space-y-6 p-6">
        {' '}
        <Button variant="outline" onClick={onBack}>
          Назад{' '}
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

            <Button>Создать</Button>
          </CardContent>
        </Card>
      </div>
    )
  }

  if (isLoading) {
    return (
      <div className="mx-auto max-w-2xl p-6">
        {' '}
        <Button variant="outline" onClick={onBack}>
          Назад{' '}
        </Button>
        <p className="mt-4">Loading...</p>
      </div>
    )
  }

  if (!commitment) {
    return (
      <div className="mx-auto max-w-2xl p-6">
        {' '}
        <Button variant="outline" onClick={onBack}>
          Назад{' '}
        </Button>
        <p className="mt-4">Commitment not found</p>
      </div>
    )
  }

  return (
    <div className="mx-auto max-w-2xl space-y-6 p-6">
      {' '}
      <Button variant="outline" onClick={onBack}>
        Назад{' '}
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
