import { tx } from "@hirosystems/clarinet-sdk";
import { Cl } from "@stacks/transactions";
import fc from "fast-check";
import { expect } from "vitest";
import { Model, Real } from "./types";

export const CounterAddErr = (accounts: Map<string, string>) => fc
  .record({
    number: fc.integer({ min: 0, max: 2 }),
    sender: fc.constantFrom(...accounts.values()),
  })
  .map((r) => ({
    check: (_model: Readonly<Model>) => {
      return r.number < 2;
    },
    run: (_model: Model, real: Real) => {
      const block = real.simnet.mineBlock([
        tx.callPublicFn("counter", "add", [Cl.uint(r.number)], r.sender),
      ]);
      expect(block[0].result).toBeErr(Cl.uint(402));

      console.log(
        `Ӿ tx-sender ${r.sender.padStart(41, " ")} ✓ ${
          "add".padStart(11, " ")
        } ${r.number.toString().padStart(10, " ")} ERROR_ADD_MORE_THAN_ONE`,
      );
    },
    toString: () => `add ${r.number}`,
  }));
