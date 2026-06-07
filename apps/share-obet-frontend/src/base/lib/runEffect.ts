import { Effect } from 'effect'
import { AppRuntime, type RuntimeServices } from '../_setup'

export const runEffect = <
  A,
  E,
>(
  effect: Effect.Effect<
    A,
    E,
    RuntimeServices
  >,
) =>
  AppRuntime.runPromise(effect)