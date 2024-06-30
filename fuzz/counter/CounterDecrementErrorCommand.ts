import { expect } from "vitest";
import { CounterCommand, Principal, Real, Stub } from "./CounterCommandModel.ts";
import { Cl, ClarityType } from "@stacks/transactions";

export class CounterDecrementErrorCommand implements CounterCommand {
  readonly sender: Principal;

  constructor(sender: Principal) {
    this.sender = sender;
  }

  check(model: Readonly<Stub>): boolean {
    return model.counter < 1;
  }

  run(_: Stub, real: Real): void {
    const { result } = real.simnet.callPublicFn("counter", "decrement", [], this.sender.value);
    expect(result).toHaveClarityType(ClarityType.ResponseErr);
    expect(result).toBeErr(Cl.uint(401));

    console.log(
      `Ӿ tx-sender ${this.sender.value.padStart(41, " ")}  ✓ ${"decrement".padStart(
        11,
        " "
      )} ERR_COUNTER_MUST_BE_POSITIVE`
    );
  }

  toString() {
    return `decrement`;
  }
}
