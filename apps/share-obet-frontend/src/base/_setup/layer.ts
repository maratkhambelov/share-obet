import { Layer } from 'effect'
import { ApiConfigLive } from './apiConfig'
import { HttpClientLive } from './httpClient.ts'

export const ApiLayer = Layer.mergeAll(ApiConfigLive, HttpClientLive)
