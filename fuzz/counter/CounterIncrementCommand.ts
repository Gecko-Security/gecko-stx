import { expect } from "vitest";
import { CounterCommand, Principal, Real, Stub } from "./CounterCommandModel.ts";
import { Cl } from "@stacks/transactions";

export class CounterIncrementCommand implements CounterCommand {
  readonly sender: Principal;

  constructor(sender: Principal) {
    this.sender = sender;
  }

  check(_: Readonly<Stub>): boolean {
    return true;
  }

  run(model: Stub, real: Real): void {
    const { result } = real.simnet.callPublicFn("counter", "increment", [], this.sender.value);

    expect(result).toBeOk(Cl.bool(true));

    model.counter = model.counter + 1;

    console.log(
      `Ӿ tx-sender ${this.sender.value.padStart(41, " ")} ✓ ${"increment".padStart(11, " ")}`
    );
  }

  toString() {
    return `increment`;
  }
}
