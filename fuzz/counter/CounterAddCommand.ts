import { Cl } from "@stacks/transactions";
import { expect } from "vitest";

import { CounterCommand, Principal, Real, Stub, Uint } from "./CounterCommandModel.ts";

export class CounterAddCommand implements CounterCommand {
  readonly number: Uint;
  readonly sender: Principal;

  constructor(number: Uint, sender: Principal) {
    this.number = number;
    this.sender = sender;
  }

  check(_: Readonly<Stub>): boolean {
    return this.number.value > 1;
  }

  run(model: Stub, real: Real): void {
    const { result } = real.simnet.callPublicFn(
      "counter",
      "add",
      [this.number.clarityValue()],
      this.sender.value
    );
    expect(result).toBeOk(Cl.bool(true));

    model.counter = model.counter + this.number.value;

    console.log(
      `Ӿ tx-sender ${this.sender.value.padStart(41, " ")} ✓ ${"add".padStart(
        11,
        " "
      )} ${this.number.value.toString().padStart(10, " ")}`
    );
  }

  toString() {
    return `add ${this.number.value}`;
  }
}
