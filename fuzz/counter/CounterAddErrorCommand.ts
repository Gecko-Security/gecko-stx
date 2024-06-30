import { Cl } from "@stacks/transactions";
import { expect } from "vitest";

import { CounterCommand, Principal, Real, Stub, Uint } from "./CounterCommandModel.ts";

export class CounterAddErrorCommand implements CounterCommand {
  readonly number: Uint;
  readonly sender: Principal;

  constructor(number: Uint, sender: Principal) {
    this.number = number;
    this.sender = sender;
  }

  check(_: Readonly<Stub>): boolean {
    return this.number.value < 2;
  }

  run(_: Stub, real: Real): void {
    const { result } = real.simnet.callPublicFn(
      "counter",
      "add",
      [this.number.clarityValue()],
      this.sender.value
    );

    expect(result).toBeErr(Cl.uint(402));

    console.log(
      `Ӿ tx-sender ${this.sender.value.padStart(41, " ")} ✓ ${"add".padStart(
        11,
        " "
      )} ${this.number.value.toString().padStart(10, " ")} ERROR_ADD_MORE_THAN_ONE`
    );
  }

  toString() {
    return `add ${this.number.value}`;
  }
}
