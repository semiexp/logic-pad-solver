// TODO: convert this to TypeScript

import { expect, test } from '@jest/globals';
import { solveLogicPad } from "./solverBridge";

function makeTilesData(data) {
  const ret = []
  for (let y = 0; y < data.length; ++y) {
    const row = [];
    for (let x = 0; x < data[y].length; ++x) {
      const c = data[y].charAt(x);

      if (c == "O") {
        row.push({ exists: true, fixed: true, color: "light" });
      } else if (c == "#") {
        row.push({ exists: true, fixed: true, color: "dark" });
      } else if (c == ".") {
        row.push({ exists: true, fixed: false, color: "gray" });
      } else if (c == " ") {
        row.push({ exists: false, fixed: false, color: "gray" });
      }
    }
    ret.push(row);
  }
  return ret;
}

function makeExpectedData(data) {
  const ret = []
  for (let y = 0; y < data.length; ++y) {
    const row = [];
    for (let x = 0; x < data[y].length; ++x) {
      const c = data[y].charAt(x);

      if (c == "O") {
        row.push("light");
      } else if (c == "#") {
        row.push("dark");
      } else {
        row.push(null);
      }
    }
    ret.push(row);
  }
  return ret;
}

test("connectAll", () => {
  const puzzle = {
    width: 5,
    height: 3,
    connections: [],
    tiles: makeTilesData([
      " ....",
      "..#..",
      "O.#.O",
    ]),
    rules: [
      { type: "connectAll", color: "light" },
    ],
  };
  expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
    " OOO.",
    ".O#..",
    "O.#.O",
  ]));
});
