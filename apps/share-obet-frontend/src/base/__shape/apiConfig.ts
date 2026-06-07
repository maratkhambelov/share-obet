import { Context } from 'effect'

export interface ApiConfigShape {
  readonly apiUrl: string | undefined
}

export class ApiConfig extends Context.Tag('ApiConfig')<ApiConfig, ApiConfigShape>() {}

