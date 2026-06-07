import { Context } from 'effect'

export interface ApiConfigShape {
  readonly apiUrl: string | undefined
}

export class ApiConfig extends Context.Tag('ApiConfig')<ApiConfig, ApiConfigShape>() {}

import { Layer } from 'effect'
import { env } from './env.ts'

export const ApiConfigLive = Layer.succeed(ApiConfig, {
  apiUrl: env.apiUrl,
})
