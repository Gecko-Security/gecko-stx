import { initSimnet, tx } from "@hirosystems/clarinet-sdk";
import { Cl } from "@stacks/transactions";
import fc from "fast-check";
import { describe, expect, it } from "vitest";

import { counterCommands } from "./CounterCommands";

const accounts = simnet.getAccounts();
const address1 = accounts.get("wallet_1")!;

describe("manual tests", () => {
  it("ensures that <get-counter> send the counter value", () => {
    const { result } = simnet.callReadOnlyFn("counter", "get-counter", [], address1);

    expect(result).toBeUint(0);
  });

  it("ensures that <increment> adds 1", () => {
    const { result } = simnet.callPublicFn("counter", "increment", [], address1);
    expect(result).toBeOk(Cl.bool(true));

    const counter = simnet.getDataVar("counter", "counter");
    expect(counter).toBeUint(1);
  });

  it("ensures that <decrement> removes 1", () => {
    simnet.callPublicFn("counter", "increment", [], address1);
    const { result } = simnet.callPublicFn("counter", "decrement", [], address1);
    expect(result).toBeOk(Cl.bool(true));

    const counter = simnet.getDataVar("counter", "counter");
    expect(counter).toBeUint(0);
  });

  it("ensures that <decrement> throws an error if result is lower than 0", () => {
    const block = simnet.mineBlock([tx.callPublicFn("counter", "decrement", [], address1)]);

    expect(block[0].result).toBeErr(Cl.uint(401));
  });

  it("ensures that <add> adds up the right amout", () => {
    const { result } = simnet.callPublicFn("counter", "add", [Cl.uint(3)], address1);

    expect(result).toBeOk(Cl.bool(true));

    const counter = simnet.getDataVar("counter", "counter");
    expect(counter).toBeUint(3);
  });

  it("ensures that <add> throws an error if n is too low", () => {
    const block = simnet.mineBlock([tx.callPublicFn("counter", "add", [Cl.uint(1)], address1)]);

    expect(block[0].result).toBeErr(Cl.uint(402));
  });
});

// // https://github.com/dubzzz/fast-check/discussions/3026#discussioncomment-3875818

describe("invariant tests", () => {
  it("runs invariant test", async () => {
    const initialChain = { simnet: await initSimnet() };
    const initialModel = {
      counter: 0,
    };

    fc.assert(
      // @ts-ignore
      fc.property(counterCommands(), (cmds: []) => {
        const state = () => ({
          model: initialModel,
          real: initialChain,
        });
        fc.modelRun(state, cmds);
      }),
      { numRuns: 100, verbose: true }
    );
  });
});