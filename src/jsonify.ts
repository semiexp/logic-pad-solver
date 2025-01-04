import { Serializer } from './logic-pad/src/data/serializer/allSerializers';
import { Compressor } from './logic-pad/src/data/serializer/compressor/allCompressors';
import { Puzzle } from './logic-pad/src/data/puzzle';
import ConnectAllRule from './logic-pad/src/data/rules/connectAllRule';
import BanPatternRule from './logic-pad/src/data/rules/banPatternRule';
import UndercluedRule from './logic-pad/src/data/rules/undercluedRule';
import SameShapeRule from './logic-pad/src/data/rules/sameShapeRule';
import UniqueShapeRule from './logic-pad/src/data/rules/uniqueShapeRule';
import RegionAreaRule from './logic-pad/src/data/rules/regionAreaRule';
import CellCountRule from './logic-pad/src/data/rules/cellCountRule';

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
    } else if (rule instanceof SameShapeRule) {
      rules.push({
        type: "sameShape",
        color: rule.color,
      });
    } else if (rule instanceof UniqueShapeRule) {
      rules.push({
        type: "uniqueShape",
        color: rule.color,
      });
    } else if (rule instanceof RegionAreaRule) {
      rules.push({
        type: "regionArea",
        color: rule.color,
        size: rule.size,
      });
    } else if (rule instanceof CellCountRule) {
      rules.push({
        type: "cellCount",
        color: rule.color,
        count: rule.count,
      });
    } else if (rule instanceof UndercluedRule) {
      continue;
    } else {
      throw new Error(`Unknown rule type (${rule.explanation})`);
    }
  }

  for (const [rule, symbols] of puzzle.grid.symbols) {
    if (rule === "minesweeper") {
      rules.push({
        type: "minesweeper",
        tiles: symbols,
      });
    } else if (rule === "number") {
      rules.push({
        type: "number",
        tiles: symbols,
      });
    } else if (rule === "letter") {
      rules.push({
        type: "letter",
        tiles: symbols,
      });
    } else if (rule === "dart") {
      rules.push({
        type: "dart",
        tiles: symbols,
      });
    } else if (rule === "viewpoint") {
      rules.push({
        type: "viewpoint",
        tiles: symbols,
      });
    } else if (rule === "lotus") {
      const tiles = symbols.map((symbol) => {
        const s = symbol as LotusSymbol;
        return {
          x: Math.round(s.x * 2),
          y: Math.round(s.y * 2),
          orientation: s.orientation,
        };
      });

      rules.push({
        type: "lotus",
        tiles,
      });
    } else if (rule === "galaxy") {
      const tiles = symbols.map((symbol) => {
        const s = symbol as GalaxySymbol;
        return {
          x: Math.round(s.x * 2),
          y: Math.round(s.y * 2),
        };
      });

      rules.push({
        type: "galaxy",
        tiles,
      });
    } else {
      throw new Error(`Unknown symbol type: ${rule}`);
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
