import fc from "fast-check";
import { tx } from "@hirosystems/clarinet-sdk";
import { expect } from "vitest";
import { Cl } from "@stacks/transactions";
import { Model, Real } from "./types";

export const CounterIncrement = (accounts: Map<string, string>) => fc
  .record({
    sender: fc.constantFrom(...accounts.values()),
  })
  .map((r) => ({
    check: (_model: Readonly<Model>) => {
      return true;
    },
    run: (model: Model, real: Real) => {
      const block = real.simnet.mineBlock([
        tx.callPublicFn("counter", "increment", [], r.sender),
      ]);
      expect(block[0].result).toBeOk(Cl.bool(true));

      model.counter = model.counter + 1;

      console.log(
        `Ӿ tx-sender ${r.sender.padStart(41, " ")} ✓ ${
          "increment".padStart(11, " ")
        }`,
      );
    },
    toString: () => `increment`,
  }));
