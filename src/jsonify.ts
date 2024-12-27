import { Serializer } from './logic-pad/src/data/serializer/allSerializers';
import { Compressor } from './logic-pad/src/data/serializer/compressor/allCompressors';
import { Puzzle } from './logic-pad/src/data/puzzle';
import ConnectAllRule from './logic-pad/src/data/rules/connectAllRule';
import BanPatternRule from './logic-pad/src/data/rules/banPatternRule';

export async function urlToPuzzle(url: string): Promise<Puzzle> {
  const value = decodeURIComponent(url).split("?d=")[1];
  const decompressed = await Compressor.decompress(value);
  return Serializer.parsePuzzle(decompressed);
}

export function puzzleToJson(puzzle: Puzzle): string {
  const tiles = puzzle.grid.tiles;
  const rules = [];

  for (const rule of puzzle.grid.rules) {
    if (rule instanceof ConnectAllRule) {
      rules.push({
        type: "connectAll",
        color: rule.color,
      });
    } else if (rule instanceof BanPatternRule) {
      rules.push({
        type: "forbiddenPattern",
        pattern: rule.pattern.tiles,
      });
    }
  }

  for (const [rule, symbols] of puzzle.grid.symbols) {
    if (rule === "minesweeper") {
      rules.push({
        type: "minesweeper",
        tiles: symbols,
      });
    }
  }

  return JSON.stringify({
    width: puzzle.grid.width,
    height: puzzle.grid.height,
    connections: puzzle.grid.connections.edges,
    tiles,
    rules,
  });
}
