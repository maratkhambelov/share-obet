import { Layer, ManagedRuntime } from 'effect'
import { HttpClientLive } from '../../base/setup'
import { ApiConfigLive } from '../../base/setup'

export const ApiLayer =
  Layer.mergeAll(
    ApiConfigLive,
    HttpClientLive,
  );

export const AppRuntime =
  ManagedRuntime.make(
    ApiLayer,
  );