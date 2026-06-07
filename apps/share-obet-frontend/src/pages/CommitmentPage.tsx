export const CommitmentPage = ({
  commitmentId,
  onBack,
}: {
  commitmentId?: string
  onBack: () => void
}) => {
  return (
    <div>
      <button onClick={() => onBack()}>Back</button>

      <h1>Commitment {commitmentId}</h1>
    </div>
  )
}

//type CreateCommitmentPageProps = {
//     onBack: () => void;
// };
//
// function CreateCommitmentPage({
//                                   onBack,
//                               }: CreateCommitmentPageProps) {
//     return (
//         <div>
//             <button onClick={onBack}>
//             Back
//             </button>
//
//             <h1>Create Commitment</h1>
//     </div>
// );
// }
