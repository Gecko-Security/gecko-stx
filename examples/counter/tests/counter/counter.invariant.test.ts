import fc from "fast-check";
import { it } from "vitest";

import { CounterAdd } from "./CounterAdd";
import { CounterAddErr } from "./CounterAddErr";
import { CounterDecrement } from "./CounterDecrement";
import { CounterDecrementErr } from "./CounterDecrementErr";
import { CounterIncrement } from "./CounterIncrement";
import { CounterGet } from "./CounterGet";

it("runs invariant test", async () => {
  const accounts = simnet.getAccounts();

  const invariants = [
    CounterAdd(accounts),
    CounterAddErr(accounts),
    CounterDecrement(accounts),
    CounterDecrementErr(accounts),
    CounterGet(accounts),
    CounterIncrement(accounts),
  ];

  const model = {
    counter: 0,
  };

  fc.assert(
    fc.property(
      fc.commands(invariants, { size: "+1" }),
      (cmds) => {
        const state = () => ({ model: model, real: { simnet } });
        fc.modelRun(state, cmds);
      },
    ),
    { numRuns: 100, verbose: 2 },
  );
});
