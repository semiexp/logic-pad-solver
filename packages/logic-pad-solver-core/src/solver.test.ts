import { describe, expect, test } from '@jest/globals';
import { PuzzleData, solveLogicPad, Tile } from "./solverBridge.js";

function makeTilesData(data: string[]): Tile[][] {
  const ret: Tile[][] = []
  for (let y = 0; y < data.length; ++y) {
    const row: Tile[] = [];
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

function makeExpectedData(data: string[]): ("dark" | "light" | null)[][] {
  const ret: ("dark" | "light" | null)[][] = []
  for (let y = 0; y < data.length; ++y) {
    const row: ("dark" | "light" | null)[] = [];
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
  const puzzle: PuzzleData = {
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

test("connectAllBoth", () => {
  const puzzle: PuzzleData = {
    width: 5,
    height: 4,
    connections: [],
    tiles: makeTilesData([
      "..O..",
      ".....",
      "#....",
      "..#O.",
    ]),
    rules: [
      { type: "connectAll", color: "light" },
      { type: "connectAll", color: "dark" },
    ],
  };
  expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
    "..OOO",
    "....O",
    "#...O",
    "###OO",
  ]));
});

test("forbiddenPattern", () => {
  const puzzle: PuzzleData = {
    width: 5,
    height: 4,
    connections: [],
    tiles: makeTilesData([
      ".....",
      "...O.",
      "O....",
      ".....",
    ]),
    rules: [
      {
        type: "forbiddenPattern",
        pattern: [
          [{ exists: true, fixed: true, color: "light" }],
          [{ exists: true, fixed: true, color: "light" }],
        ]
      },
    ],
  };
  expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
    "...#.",
    "#.#O#",
    "O#.#.",
    "#....",
  ]));
});

test("sameShape", () => {
  const puzzle: PuzzleData = {
    width: 5,
    height: 4,
    connections: [],
    tiles: makeTilesData([
      "##..O",
      "#OOO.",
      "O....",
      "..###",
    ]),
    rules: [
      { type: "sameShape", color: "dark" },
    ],
  };
  expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
    "###.O",
    "#OOOO",
    "O..O.",
    "O.###",
  ]));
});

test("uniqueShape", () => {
  const puzzle: PuzzleData = {
    width: 5,
    height: 4,
    connections: [],
    tiles: makeTilesData([
      ".O#O.",
      "##.##",
      ".#...",
      ".O#..",
    ]),
    rules: [
      { type: "uniqueShape", color: "light" },
    ],
  };
  expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
    ".O#O.",
    "##.##",
    "O#...",
    "OO#..",
  ]));
});

test("uniqueShape", () => {
  const puzzle: PuzzleData = {
    width: 5,
    height: 4,
    connections: [],
    tiles: makeTilesData([
      "#..#.",
      ".....",
      ".#..#",
      ".....",
    ]),
    rules: [
      { type: "regionArea", color: "dark", size: 3 },
    ],
  };
  expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
    "##O#.",
    "#O.#O",
    "O#.O#",
    ".#O##",
  ]));
});

test("minesweeper", () => {
  const puzzle: PuzzleData = {
    width: 5,
    height: 4,
    connections: [],
    tiles: makeTilesData([
      ".....",
      ".O...",
      ".#...",
      ".....",
    ]),
    rules: [
      {
        type: "minesweeper",
        tiles: [
          { x: 1, y: 1, number: 1 },
        ]
      },
    ],
  };
  expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
    "OOO..",
    "OOO..",
    "O#O..",
    ".....",
  ]));
});

test("number", () => {
  const puzzle: PuzzleData = {
    width: 5,
    height: 4,
    connections: [],
    tiles: makeTilesData([
      ".#...",
      ".....",
      ".....",
      ".....",
    ]),
    rules: [
      {
        type: "number",
        tiles: [
          { x: 1, y: 0, number: 4 },
          { x: 2, y: 0, number: 7 },
          { x: 1, y: 1, number: 5 },
        ]
      },
    ],
  };
  expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
    "##OO.",
    "#O#..",
    "#O..O",
    "OO...",
  ]));
});

test("letter", () => {
  const puzzle: PuzzleData = {
    width: 5,
    height: 4,
    connections: [],
    tiles: makeTilesData([
      ".#...",
      ".....",
      ".....",
      ".....",
    ]),
    rules: [
      {
        type: "letter",
        tiles: [
          { x: 1, y: 0, letter: "A" },
          { x: 1, y: 1, letter: "B" },
          { x: 1, y: 3, letter: "A" },
          { x: 4, y: 3, letter: "B" },
        ]
      },
    ],
  };
  expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
    "##...",
    "#O...",
    "#....",
    ".#..O",
  ]));
});

test("dart", () => {
  const puzzle: PuzzleData = {
    width: 5,
    height: 4,
    connections: [],
    tiles: makeTilesData([
      ".....",
      "#....",
      ".....",
      ".....",
    ]),
    rules: [
      {
        type: "dart",
        tiles: [
          { x: 0, y: 1, orientation: "right", number: 4 },
          { x: 2, y: 3, orientation: "up-left", number: 0 },
        ]
      },
    ],
  };
  expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
    ".....",
    "#OOOO",
    ".#...",
    "..#..",
  ]));
});

test("viewpoint", () => {
  const puzzle: PuzzleData = {
    width: 5,
    height: 4,
    connections: [],
    tiles: makeTilesData([
      ".....",
      "#....",
      ".....",
      ".....",
    ]),
    rules: [
      {
        type: "viewpoint",
        tiles: [
          { x: 0, y: 1, number: 3 },
          { x: 2, y: 1, number: 7 },
        ]
      },
    ],
  };
  expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
    "..O..",
    "#OOOO",
    "#.O..",
    "..O..",
  ]));
});

describe("lotus", () => {
  test("cell", () => {
    const puzzle: PuzzleData = {
      width: 5,
      height: 4,
      connections: [],
      tiles: makeTilesData([
        ".....",
        ".##..",
        ".O...",
        ".....",
      ]),
      rules: [
        {
          type: "lotus",
          tiles: [
            { x: 4, y: 2, orientation: "up" },  // on the cell (x=2, y=1)
          ]
        },
      ],
    };
    expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
      ".....",
      ".###.",
      ".O.O.",
      ".....",
    ]));
  });

  test("edge", () => {
    const puzzle: PuzzleData = {
      width: 5,
      height: 4,
      connections: [],
      tiles: makeTilesData([
        ".....",
        ".##..",
        ".O...",
        ".....",
      ]),
      rules: [
        {
          type: "lotus",
          tiles: [
            { x: 5, y: 2, orientation: "up" },  // the edge between the cell (x=2, y=1) and (x=3, y=1)
          ]
        },
      ],
    };
    expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
      ".....",
      "O####",
      ".O..O",
      ".....",
    ]));
  });

  test("edge-invalid", () => {
    const puzzle: PuzzleData = {
      width: 5,
      height: 4,
      connections: [],
      tiles: makeTilesData([
        ".....",
        ".##..",
        ".....",
        ".....",
      ]),
      rules: [
        {
          type: "lotus",
          tiles: [
            { x: 4, y: 3, orientation: "up" },  // the edge between the cell (x=2, y=1) and (x=2, y=2)
          ]
        },
      ],
    };
    expect(solveLogicPad(puzzle, true)).toEqual({
      "error": "lotus on invalid position",
    })
  });

  test("corner", () => {
    const puzzle: PuzzleData = {
      width: 5,
      height: 4,
      connections: [],
      tiles: makeTilesData([
        ".....",
        ".##..",
        ".....",
        ".....",
      ]),
      rules: [
        {
          type: "lotus",
          tiles: [
            { x: 3, y: 3, orientation: "up" },  // the corner to the bottom-right of the cell (x=1, y=1)
          ]
        },
      ],
    };
    expect(solveLogicPad(puzzle, true)).toEqual({
      "error": "lotus on invalid position",
    })
  });
});

describe("galaxy", () => {
  test("cell", () => {
    const puzzle: PuzzleData = {
      width: 5,
      height: 4,
      connections: [],
      tiles: makeTilesData([
        ".....",
        ".##..",
        ".#...",
        ".....",
      ]),
      rules: [
        {
          type: "galaxy",
          tiles: [
            { x: 4, y: 2 },  // on the cell (x=2, y=1)
          ]
        },
      ],
    };
    expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
      "...#.",
      ".###.",
      ".#...",
      ".O...",
    ]));
  });

  test("edge", () => {
    const puzzle: PuzzleData = {
      width: 5,
      height: 4,
      connections: [],
      tiles: makeTilesData([
        ".....",
        ".##..",
        ".#...",
        ".....",
      ]),
      rules: [
        {
          type: "galaxy",
          tiles: [
            { x: 5, y: 2 },  // on the edge between the cell (x=2, y=1) and (x=3, y=1)
          ]
        },
      ],
    };
    expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
      "....#",
      "O####",
      "O#...",
      ".O...",
    ]));
  });

  test("corner", () => {
    const puzzle: PuzzleData = {
      width: 5,
      height: 4,
      connections: [],
      tiles: makeTilesData([
        ".....",
        ".##..",
        ".#...",
        ".....",
      ]),
      rules: [
        {
          type: "galaxy",
          tiles: [
            { x: 3, y: 3 },  // on the corner to the bottom-right of the cell (x=1, y=1)
          ]
        },
      ],
    };
    expect(solveLogicPad(puzzle, true)).toEqual({
      "error": "galaxies on corners may cause unexpected behavior",
    });
  });
});

test("number-offbyx", () => {
  const puzzle: PuzzleData = {
    width: 5,
    height: 4,
    connections: [],
    tiles: makeTilesData([
      ".#...",
      ".....",
      ".....",
      ".....",
    ]),
    rules: [
      {
        type: "number",
        tiles: [
          { x: 1, y: 0, number: 10 },
          { x: 2, y: 0, number: 5 },
          { x: 1, y: 1, number: 7 },
        ]
      },
      {
        type: "offByX",
        number: 1,
      }
    ],
  };
  expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
    "##O..",
    "#OO..",
    "#....",
    ".....",
  ]));
});

describe("symbolCount", () => {
  test("exactly", () => {
    const puzzle: PuzzleData = {
      width: 5,
      height: 4,
      connections: [],
      tiles: makeTilesData([
        ".#...",
        ".....",
        ".....",
        ".....",
      ]),
      rules: [
        {
          type: "viewpoint",
          tiles: [
            { x: 1, y: 0, number: 2 },
            { x: 1, y: 1, number: 2 },
            { x: 3, y: 1, number: 6 },
          ]
        },
        {
          type: "symbolCount",
          number: 1,
          kind: "exactly",
          color: "dark",
        }
      ],
    };
    expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
      "##O#.",
      ".O###",
      "...#.",
      "...#.",
    ]));
  });

  test("atLeast", () => {
    const puzzle: PuzzleData = {
      width: 5,
      height: 4,
      connections: [],
      tiles: makeTilesData([
        "..O..",
        ".#O#.",
        "..O..",
        ".....",
      ]),
      rules: [
        {
          type: "viewpoint",
          tiles: [
            { x: 1, y: 1, number: 2 },
            { x: 3, y: 1, number: 2 },
          ]
        },
        {
          type: "symbolCount",
          number: 2,
          kind: "atLeast",
          color: "dark",
        }
      ],
    };
    expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
      ".OOO.",
      "##O##",
      "#OOO#",
      "#####",
    ]));
  });

  test("atMost", () => {
    const puzzle: PuzzleData = {
      width: 5,
      height: 4,
      connections: [],
      tiles: makeTilesData([
        "..O..",
        ".#...",
        ".....",
        ".....",
      ]),
      rules: [
        {
          type: "viewpoint",
          tiles: [
            { x: 1, y: 1, number: 2 },
            { x: 2, y: 1, number: 2 },
            { x: 2, y: 2, number: 2 },
          ]
        },
        {
          type: "symbolCount",
          number: 1,
          kind: "atMost",
          color: "dark",
        }
      ],
    };
    expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
      "..O..",
      ".#O#.",
      ".O#..",
      ".....",
    ]));
  });
});

describe("symbolCountWithGalaxy", () => {
  test("exactly", () => {
    const puzzle: PuzzleData = {
      width: 5,
      height: 4,
      connections: [],
      tiles: makeTilesData([
        ".....",
        ".....",
        "...#.",
        "...#.",
      ]),
      rules: [
        {
          type: "galaxy",
          tiles: [
            { x: 6, y: 4 },
          ]
        },
        {
          type: "symbolCount",
          number: 1,
          kind: "exactly",
          color: "dark",
        }
      ],
    };
    expect(solveLogicPad(puzzle, true)).toEqual(makeExpectedData([
      "OOOOO",
      "OO.#.",
      "OO.#.",
      "OO.#.",
    ]));
  });
});
