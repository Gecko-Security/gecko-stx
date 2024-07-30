import { tx } from "@hirosystems/clarinet-sdk";
import { Cl } from "@stacks/transactions";
import fc from "fast-check";
import { expect } from "vitest";
import { Model, Real } from "./types";

export const CounterDecrement = (accounts: Map<string, string>) => fc
  .record({
    sender: fc.constantFrom(...accounts.values()),
  })
  .map((r) => ({
    check: (model: Readonly<Model>) => {
      return model.counter > 0;
    },
    run: (model: Model, real: Real) => {
      const block = real.simnet.mineBlock([
        tx.callPublicFn("counter", "decrement", [], r.sender),
      ]);
      expect(block[0].result).toBeOk(Cl.bool(true));

      model.counter = model.counter - 1;

      console.log(
        `Ӿ tx-sender ${r.sender.padStart(41, " ")} ✓ ${
          "decrement".padStart(11, " ")
        }`,
      );
    },
    toString: () => `decrement`,
  }));
