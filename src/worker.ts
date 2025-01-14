import { solveLogicPad } from "logic-pad-solver-core";

export function solve(data: string, underclued: boolean): string {
  return solveLogicPad(data, underclued);
}
