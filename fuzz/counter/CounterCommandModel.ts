import { Simnet } from "@hirosystems/clarinet-sdk";
import { Cl, PrincipalCV, UIntCV } from "@stacks/transactions";
import fc from "fast-check";

export type Stub = {
  counter: number;
};

export type Real = {
  simnet: Simnet;
};

export type CounterCommand = fc.Command<Stub, Real>;

// Interop: Clarity <-> TypeScript

export class Uint {
  readonly value: number;

  constructor(value: number) {
    this.value = value;
  }

  clarityValue(): UIntCV {
    return Cl.uint(this.value);
  }
}

export class Principal {
  readonly value: string;

  constructor(value: string) {
    this.value = value;
  }

  clarityValue(): PrincipalCV {
    if (this.value.includes(".")) {
      return Cl.contractPrincipal(...(this.value.split(".") as [string, string]));
    }
    return Cl.standardPrincipal(this.value);
  }
}
