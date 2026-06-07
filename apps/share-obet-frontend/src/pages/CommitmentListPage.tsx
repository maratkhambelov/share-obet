import { useEffect, useState } from 'react'

import { AppRuntime } from '@/App/providers'

import { getCommitment } from '@/entities/commitment'
import { getCommitments } from '@/entities/commitment'

import type { CommitmentBase } from '@/entities/commitment'
import type { CommitmentDetail } from '@/entities/commitment'

export const CommitmentListPage = () => {
  const [commitments, setCommitments] = useState<ReadonlyArray<CommitmentBase>>(
    [],
  )

  const [selectedCommitment, setSelectedCommitment] =
    useState<CommitmentDetail | null>(null)

  const [isLoading, setIsLoading] = useState(false)

  useEffect(() => {
    AppRuntime.runPromise(getCommitments)
      .then(setCommitments)
      .catch(console.error)
  }, [])

  const handleCommitmentClick = async (id: string) => {
    try {
      setIsLoading(true)

      const commitment = await AppRuntime.runPromise(getCommitment(id))

      setSelectedCommitment(commitment)
    } catch (error) {
      console.error(error)
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <div>
      <h1>Commitments</h1>

      <ul>
        {commitments.map((commitment) => (
          <li key={commitment.id}>
            <button
              type="button"
              onClick={() => handleCommitmentClick(commitment.id)}
            >
              {commitment.title}
            </button>
          </li>
        ))}
      </ul>

      {isLoading && <p>Loading...</p>}

      {selectedCommitment && (
        <section>
          <h2>{selectedCommitment.title}</h2>

          <p>{selectedCommitment.description}</p>

          <p>Status: {selectedCommitment.status}</p>

          <p>End date: {selectedCommitment.end_date}</p>

          <p>Created at: {selectedCommitment.created_at}</p>
        </section>
      )}
    </div>
  )
}
