import { tx } from "@hirosystems/clarinet-sdk";
import { Cl } from "@stacks/transactions";
import fc from "fast-check";
import { expect } from "vitest";
import { Model, Real } from "./types";

export const CounterDecrementErr = (accounts: Map<string, string>) => fc
  .record({
    sender: fc.constantFrom(...accounts.values()),
  })
  .map((r) => ({
    check: (model: Readonly<Model>) => {
      return model.counter < 0;
    },
    run: (_model: Model, real: Real) => {
      const block = real.simnet.mineBlock([
        tx.callPublicFn("counter", "decrement", [], r.sender),
      ]);
      expect(block[0].result).toBeErr(Cl.uint(403));

      console.log(
        `Ӿ tx-sender ${r.sender.padStart(41, " ")} ✓ ${
          "decrement".padStart(11, " ")
        } ERR_COUNTER_MUST_BE_POSITIVE`,
      );
    },
    toString: () => `decrement`,
  }));
