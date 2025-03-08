import { solveLogicPad, PuzzleData, SolverResult } from "logic-pad-solver-core";

export function solve(data: PuzzleData, underclued: boolean): SolverResult {
  return solveLogicPad(data, underclued);
}
