export type Color = "dark" | "light" | "gray";
export type Orientation = "up" | "up-right" | "right" | "down-right" | "down" | "down-left" | "left" | "up-left";

export interface Tile {
  exists: boolean;
  fixed: boolean;
  color: Color
}

/*
Note: for "lotus" and "galaxy" rules, the x and y coordinates should be multiplied by 2
so that we can handle these symbols on edges or corners of tiles (see `logic-pad-solver/src/jsonify.ts` as an example).
For other rules, the x and y coordinates should be as is.
TODO: clarify what happens when x and y are not integers.
*/
export type Rule =
    { type: "connectAll"; color: Color }
  | { type: "forbiddenPattern"; pattern: Tile[][] }
  | { type: "sameShape"; color: Color }
  | { type: "uniqueShape"; color: Color }
  | { type: "regionArea"; color: Color; size: number }
  | { type: "cellCount"; color: Color; count: number }
  | { type: "offByX"; number: number }
  | { type: "symbolCount"; number: number; kind: "exactly" | "atMost" | "atLeast"; color: Color }
  | { type: "minesweeper"; tiles: readonly { x: number; y: number; number: number }[] }
  | { type: "number"; tiles: readonly { x: number; y: number; number: number }[] }
  | { type: "letter"; tiles: readonly { x: number; y: number; letter: string }[] }
  | { type: "dart"; tiles: readonly { x: number; y: number; orientation: Orientation; number: number }[]}
  | { type: "viewpoint"; tiles: readonly { x: number; y: number; number: number }[] }
  | { type: "lotus"; tiles: readonly { x: number; y: number; orientation: Orientation }[] }
  | { type: "galaxy"; tiles: readonly { x: number; y: number; }[] }

export interface PuzzleData {
  width: number;
  height: number;
  connections: { x1: number; y1: number; x2: number; y2: number }[];
  tiles: Tile[][];
  rules: Rule[];
}

export type SolverResult = { error: string } | null | ("dark" | "light" | null)[][];

export declare function solveLogicPad(data: PuzzleData, underclued: boolean): SolverResult;
