import {
  Serializer,
  Compressor,
  ConnectAllRule,
  BanPatternRule,
  UndercluedRule,
  SameShapeRule,
  UniqueShapeRule,
  RegionAreaRule,
  CellCountRule,
  OffByXRule,
  LotusSymbol,
  GalaxySymbol,
  SymbolsPerRegionRule,
  Comparison,
  TileData,
  Color,
  MinesweeperSymbol,
  AreaNumberSymbol,
  LetterSymbol,
  DartSymbol,
  ViewpointSymbol,
} from '@logic-pad/core';
import { Puzzle } from '@logic-pad/core/data/puzzle.js';

import { PuzzleData, Rule } from "logic-pad-solver-core";

export async function urlToPuzzle(url: string): Promise<Puzzle> {
  const value = decodeURIComponent(url).split("?d=")[1];
  const decompressed = await Compressor.decompress(value);
  return Serializer.parsePuzzle(decompressed);
}

function canonizeTiles(tileData: readonly (readonly TileData[])[]): TileData[][] {
  const ret = [];
  for (const row of tileData) {
    const newRow = [];
    for (const tile of row) {
      if (tile.exists) {
        newRow.push(tile);
      } else {
        newRow.push(new TileData(true, false, Color.Gray));
      }
    }
    ret.push(newRow);
  }
  return ret;
}

export function puzzleToJson(puzzle: Puzzle): PuzzleData {
  const rules: Rule[] = [];

  for (const rule of puzzle.grid.rules) {
    if (rule instanceof ConnectAllRule) {
      rules.push({
        type: "connectAll",
        color: rule.color,
      });
    } else if (rule instanceof BanPatternRule) {
      rules.push({
        type: "forbiddenPattern",
        pattern: canonizeTiles(rule.pattern.tiles),
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
    } else if (rule instanceof OffByXRule) {
      rules.push({
        type: "offByX",
        number: rule.number,
      });
    } else if (rule instanceof UndercluedRule) {
      continue;
    } else if (rule instanceof SymbolsPerRegionRule) {
      let kind: "exactly" | "atLeast" | "atMost";
      if (rule.comparison == Comparison.Equal) {
        kind = "exactly";
      } else if (rule.comparison == Comparison.AtLeast) {
        kind = "atLeast";
      } else if (rule.comparison == Comparison.AtMost) {
        kind = "atMost";
      } else {
        throw new Error(`Unknown comparison type (${rule.comparison})`);
      }
      rules.push({
        type: "symbolCount",
        number: rule.count,
        kind,
        color: rule.color,
      });
    } else {
      throw new Error(`Unknown rule type (${rule.explanation})`);
    }
  }

  for (const [rule, symbols] of puzzle.grid.symbols) {
    if (rule === "minesweeper") {
      rules.push({
        type: "minesweeper",
        tiles: symbols as readonly MinesweeperSymbol[],
      });
    } else if (rule === "number") {
      rules.push({
        type: "number",
        tiles: symbols as readonly AreaNumberSymbol[],
      });
    } else if (rule === "letter") {
      rules.push({
        type: "letter",
        tiles: symbols as readonly LetterSymbol[],
      });
    } else if (rule === "dart") {
      rules.push({
        type: "dart",
        tiles: symbols as readonly DartSymbol[],
      });
    } else if (rule === "viewpoint") {
      rules.push({
        type: "viewpoint",
        tiles: symbols as readonly ViewpointSymbol[],
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

  const connections = puzzle.grid.connections.edges.map((edge) => {
    return {
      x1: edge.x1,
      y1: edge.y1,
      x2: edge.x2,
      y2: edge.y2,
    };
  });

  const tiles = puzzle.grid.tiles.map((row) => {
    return row.map((tile) => {
      return {
        exists: tile.exists,
        fixed: tile.fixed,
        color: tile.color,
      };
    });
  });

  return {
    width: puzzle.grid.width,
    height: puzzle.grid.height,
    connections,
    tiles,
    rules,
  };
}
