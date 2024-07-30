import { tx } from "@hirosystems/clarinet-sdk";
import { Cl } from "@stacks/transactions";
import { expect } from "vitest";
import { Model, Real } from "./types";
import fc from "fast-check";

export const CounterAdd = (accounts: Map<string, string>) => fc
  .record({
    number: fc.integer(),
    sender: fc.constantFrom(...accounts.values()),
  })
  .map((r) => ({
    check: (_model: Readonly<Model>) => {
      return r.number > 1;
    },
    run: (model: Model, real: Real) => {
      const block = real.simnet.mineBlock([
        tx.callPublicFn("counter", "add", [Cl.uint(r.number)], r.sender),
      ]);
      expect(block[0].result).toBeOk(Cl.bool(true));

      model.counter = model.counter + r.number;

      console.log(
        `Ӿ tx-sender ${r.sender.padStart(41, " ")} ✓ ${
          "add".padStart(11, " ")
        } ${r.number.toString().padStart(10, " ")}`,
      );
    },
    toString: () => `add ${r.number}`,
  }));
