import {  useState } from 'react'

import { useCommitmentsQuery, useCommitmentQuery } from '@/entities/commitment'

export const CommitmentListPage = () => {
  const [selectedId, setSelectedId] = useState<string | null>(null)

  const { data: commitments = [] } = useCommitmentsQuery()

  const { data: selectedCommitment, isLoading } = useCommitmentQuery(selectedId)

  return (
    <div>
      <h1>Commitments</h1>

      <ul>
        {commitments.map((commitment) => (
          <li key={commitment.id}>
            <button type="button" onClick={() => setSelectedId(commitment.id)}>
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
        </section>
      )}
    </div>
  )
}
