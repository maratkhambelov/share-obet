import { Context } from 'effect'
import { Layer } from 'effect'
import { env } from '../config'

export interface ApiConfigShape {
  readonly apiUrl: string | undefined
}

export class ApiConfig extends Context.Tag('ApiConfig')<ApiConfig, ApiConfigShape>() {}

export const ApiConfigLive = Layer.succeed(ApiConfig, {
  apiUrl: env.apiUrl,
})
