import { ManagedRuntime } from 'effect'
import { ApiLayer } from './layer'

export const AppRuntime = ManagedRuntime.make(ApiLayer)

export type RuntimeServices =
  typeof AppRuntime extends
    ManagedRuntime.ManagedRuntime<
      infer R,
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      infer _
    >
    ? R
    : never