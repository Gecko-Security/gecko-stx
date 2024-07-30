import { Model, Real } from "./types";
import { expect } from "vitest";
import fc from "fast-check";

export const CounterGet = (accounts: Map<string, string>) => fc
  .record({
    sender: fc.constantFrom(...accounts.values()),
  })
  .map((r) => ({
    check: (model: Readonly<Model>) => {
      return model.counter > 0;
    },
    run: (model: Model, real: Real) => {
      const getCounter = real.simnet.callReadOnlyFn(
        "counter",
        "get-counter",
        [],
        r.sender,
      );

      expect(getCounter.result).toBeUint(model.counter);

      console.log(
        `Ӿ tx-sender ${r.sender.padStart(41, " ")} ✓ ${
          "get-counter".padStart(11, " ")
        }`,
      );
    },
    toString: () => `get-counter`,
  }));
