import fc from "fast-check";

import { Principal, Uint } from "./CounterCommandModel.ts";

import { CounterAddCommand } from "./CounterAddCommand.ts";
import { CounterAddErrorCommand } from "./CounterAddErrorCommand.ts";
import { CounterDecrementCommand } from "./CounterDecrementCommand.ts";
import { CounterDecrementErrorCommand } from "./CounterDecrementErrorCommand.ts";
import { CounterGetCommand } from "./CounterGetCommand.ts";
import { CounterIncrementCommand } from "./CounterIncrementCommand.ts";

const getAccounts = () => simnet.getAccounts().values();

export function counterCommands() {
  const allCommands = [
    // CounterAddCommand
    fc
      .record({
        number: fc.integer(),
        sender: fc.constantFrom(...getAccounts()),
      })
      .map(
        (r: { number: number; sender: string }) =>
          new CounterAddCommand(new Uint(r.number), new Principal(r.sender))
      ),

    // CounterAddErrorCommand
    fc
      .record({
        number: fc.integer({ min: 0, max: 2 }),
        sender: fc.constantFrom(...getAccounts()),
      })
      .map(
        (r: { number: number; sender: string }) =>
          new CounterAddErrorCommand(new Uint(r.number), new Principal(r.sender))
      ),

    // CounterDecrementCommand
    fc
      .record({
        sender: fc.constantFrom(...getAccounts()),
      })
      .map((r: { sender: string }) => new CounterDecrementCommand(new Principal(r.sender))),

    // CounterDecrementErrorCommand
    fc
      .record({
        sender: fc.constantFrom(...getAccounts()),
      })
      .map((r: { sender: string }) => new CounterDecrementErrorCommand(new Principal(r.sender))),

    // CounterGetCommand
    fc
      .record({
        sender: fc.constantFrom(...getAccounts()),
      })
      .map((r: { sender: string }) => new CounterGetCommand(new Principal(r.sender))),

    // CounterIncrementCommand
    fc
      .record({
        sender: fc.constantFrom(...getAccounts()),
      })
      .map((r: { sender: string }) => new CounterIncrementCommand(new Principal(r.sender))),
  ];
  // On size: https://github.com/dubzzz/fast-check/discussions/2978
  return fc.commands(allCommands, { size: "+1" });
}
