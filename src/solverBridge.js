import Module from "./solver.js";

let Solver = null;

export function solve(data) {
  const dataEncoded = new TextEncoder().encode(data);
  const buf = Solver._malloc(dataEncoded.length);
  Solver.HEAPU8.set(dataEncoded, buf);

  let res = Solver._solve_puzzle(buf, dataEncoded.length);
  Solver._free(buf);

  const length = Solver.HEAPU8[res] | (Solver.HEAPU8[res + 1] << 8) | (Solver.HEAPU8[res + 2] << 16) | (Solver.HEAPU8[res + 3] << 24);

  const ansStr = new TextDecoder().decode(Solver.HEAPU8.slice(res + 4, res + 4 + length));
  return ansStr;
}

export async function loadSolver() {
  Solver = await Module();
}

export function isSolverLoaded() {
  return Solver !== null;
}
