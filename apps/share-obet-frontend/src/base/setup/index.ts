//1. ApiConfig (Tag)
// 2. HttpClient algebra
// 3. Error ADT
// 4. Schema
// 5. Feature API
// 6. TanStack Query adapter

export * from './httpClient.ts'
export * from './apiErrors.ts'
export * from './apiConfig.ts'
//import { Layer } from "effect";
//
// import { apiConfigLayer } from "./apiConfigLayer";
// import { HttpClientLive } from "./httpClientLive";
//
// export const ApiLayer =
//   Layer.mergeAll(
//     apiConfigLayer,
//     HttpClientLive,
//   );

//Effect.runPromise(
//   getCommitments.pipe(
//     Effect.provide(
//       ApiLayer,
//     ),
//   ),
// );