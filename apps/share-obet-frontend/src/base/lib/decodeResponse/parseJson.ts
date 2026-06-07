import { Effect } from "effect";
import { DecodeError } from '../../__shape'

export const parseJson = (
  response: Response,
): Effect.Effect<
  unknown,
  DecodeError
> =>
  Effect.tryPromise({
    try: () => response.json(),
    catch: cause =>
      new DecodeError({ cause }),
  })