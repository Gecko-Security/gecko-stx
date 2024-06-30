import { expect } from "vitest";
import { CounterCommand, Principal, Real, Stub } from "./CounterCommandModel.ts";
import { Cl } from "@stacks/transactions";

export class CounterDecrementCommand implements CounterCommand {
  readonly sender: Principal;

  constructor(sender: Principal) {
    this.sender = sender;
  }

  check(model: Readonly<Stub>): boolean {
    return model.counter > 0;
  }

  run(model: Stub, real: Real): void {
    const { result } = real.simnet.callPublicFn("counter", "decrement", [], this.sender.value);
    expect(result).toBeOk(Cl.bool(true));

    model.counter = model.counter - 1;

    console.log(
      `Ӿ tx-sender ${this.sender.value.padStart(41, " ")} ✓ ${"decrement".padStart(11, " ")}`
    );
  }

  toString() {
    return `decrement`;
  }
}
