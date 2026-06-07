import { useState, useMemo, useCallback, useEffect } from 'react'

// import { miniApp } from '@tma.js/sdk'
import { CommitmentListPage, CommitmentPage } from '@/pages'
import { Effect } from 'effect'

//export const runEffect = <A, E>(
//   effect: Effect.Effect<A, E>,
// ) =>
//   AppRuntime.runPromise(effect);
//useQuery({
//   queryKey: ["commitments"],
//
//   queryFn: () =>
//     AppRuntime.runPromise(
//       getCommitments,
//     ),
// });

type Screen =
  | {
      type: 'COMMITMENT_LIST'
    }
  | {
      type: 'COMMITMENT'
      commitmentId?: string
    }



//TODO: install https://effect.website/docs/getting-started/devtools/
function App() {
  const [count, setCount] = useState(0)

  const [screen, setScreen] = useState<Screen>({
    type: 'COMMITMENT_LIST',
  })
  const task = useMemo(
    () => Effect.sync(() => setCount((current) => current + 1)),
    [setCount],
  )


  const increment = useCallback(() => Effect.runSync(task), [task])

  useEffect(() => {
    // miniApp.mount();
    // miniApp.ready();
  }, [])

  switch (screen.type) {
    case 'COMMITMENT_LIST':
      return (
        <>
          <button onClick={increment}>count is {count}</button>
          <CommitmentListPage
            // onCreate={() =>
            //   setScreen({
            //     type: 'COMMITMENT',
            //   })
            // }
            // onOpen={(commitmentId) =>
            //   setScreen({
            //     type: 'COMMITMENT',
            //     commitmentId,
            //   })
            // }
          />
        </>
      )

    case 'COMMITMENT':
      return (
        <CommitmentPage
          commitmentId={screen.commitmentId}
          onBack={() =>
            setScreen({
              type: 'COMMITMENT_LIST',
            })
          }
        />
      )
  }
}

export default App



// import {useEffect} from 'react'
// import './App.css'
// // https://www.npmjs.com/package/@ton/ton?utm_source=chatgpt.com
// // https://www.npmjs.com/package/@tonconnect/ui-react?utm_source=chatgpt.com
//
// import {
//     mainButton,
//     miniApp } from "@tma.js/sdk";
// import { useLaunchParams } from "@tma.js/sdk-react";
//
//
//
// function App() {
//     const { tgWebAppData } = useLaunchParams();
//
//     const user = tgWebAppData?.user
//
//     useEffect(() => {
//         console.log(user);
//     }, [user])
//
//     useEffect(() => {
//         miniApp.mount();
//
//         miniApp.ready();
//
//         // return () => {
//         //     miniApp.unmount();
//         // };
//     }, []);
//
//     useEffect(() => {
//         mainButton.mount();
//
//         mainButton.setParams({
//             text: "Create pledge",
//             isVisible: true,
//         });
//
//         const off = mainButton.onClick(() => {
//             console.log("Create pledge click");
//         });
//
//         return () => {
//             off();
//
//             // mainButton.unmount();
//         };
//     }, []);
//
//
//
//     return (
//     <>
//         {!user && (
//             <p>
//                 Open inside Telegram
//             </p>
//         )}
//
//         PRIVET!!!!!!!!!!!!!!
//         {user && (
//             <>
//                 {user.photo_url && (
//                     <img
//                         src={user.photo_url}
//                         alt={user.first_name}
//                         width={96}
//                         height={96}
//                         style={{
//                             borderRadius: "50%",
//                         }}
//                     />
//                 )}
//
//                 <p>
//                     {user.first_name}{" "}
//                     {user.last_name}
//                 </p>
//
//                 <p>
//                     @{user.username}
//                 </p>
//
//                 <p>
//                     ID: {user.id}
//                 </p>
//
//                 <p>
//                     Premium:{" "}
//                     {user.is_premium
//                         ? "YES"
//                         : "NO"}
//                 </p>
//
//                 <p>
//                     Language:{" "}
//                     {user.language_code}
//                 </p>
//             </>
//         )}
//     </>
//   )
// }
//
// export default App
