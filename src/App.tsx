import { useState } from 'react';
import { TileData } from '@logic-pad/core';

import { urlToPuzzle, puzzleToJson } from './jsonify';
import { AnswerBoard } from './board';

type AnswerData = {
  board: readonly (readonly TileData[])[];
  answer: ("light" | "dark" | null)[][];
};

let workerInstance: any = null;

const countTilesRemaining = (board: readonly (readonly TileData[])[], answer: ("light" | "dark" | null)[][]): number => {
  let remaining = 0;

  for (let y = 0; y < board.length; y++) {
    for (let x = 0; x < board[y].length; x++) {
      if (board[y][x].exists && !board[y][x].fixed && board[y][x].color === "gray" && answer[y][x] !== null) {
        remaining++;
      }
    }
  }

  return remaining;
}

function App() {
  const [url, setUrl] = useState<string>("");
  const [answer, setAnswer] = useState<AnswerData | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isRunning, setIsRunning] = useState<boolean>(false);
  const [solverMode, setSolverMode] = useState<"solve" | "underclued" | "tilesRemaining">("solve");
  const [tilesRemaining, setTilesRemaining] = useState<number | null>(null);

  const runSolver = async () => {
    const puzzle = await urlToPuzzle(url);

    let json;
    try {
      json = puzzleToJson(puzzle);
    } catch (e: any) {
      setError(e.message);
      setAnswer(null);
      return;
    }
    
    if (workerInstance === null) {
      workerInstance = new ComlinkWorker<typeof import("./solverBridge")>(new URL("./solverBridge", import.meta.url));
    }

    const useUnderclued = solverMode === "underclued" || solverMode === "tilesRemaining";
    setIsRunning(true);
    const result = JSON.parse(await workerInstance!.solve(json, useUnderclued));
    setIsRunning(false);

    if (result === null) {
      setError("No solution found");
      setAnswer(null);
      return;
    }

    if ("error" in result) {
      setError(result.error);
      setAnswer(null);
      return;
    }

    setError(null);

    if (solverMode === "tilesRemaining") {
      const remaining = countTilesRemaining(puzzle.grid.tiles, result);
      setTilesRemaining(remaining);
      setAnswer(null);
    } else {
      setTilesRemaining(null);
      setAnswer({
        board: puzzle.grid.tiles,
        answer: result,
      });
    }
  };

  return (
    <>
      <div>
        <p>
          GitHub: <a href="https://github.com/semiexp/logic-pad-solver">semiexp/logic-pad-solver</a>
        </p>
        URL: <input type="text" value={url} onChange={e => setUrl(e.target.value)} size={40} />
        <input type="button" value="Solve" onClick={runSolver} disabled={isRunning} />
      </div>

      <div>
        <label htmlFor="solve">Solve</label>
        <input type="radio" id="solve" name="mode" value="solve" checked={solverMode === "solve"} onChange={() => setSolverMode("solve")} />

        <label htmlFor="underclued">Underclued</label>
        <input type="radio" id="underclued" name="mode" value="underclued" checked={solverMode === "underclued"} onChange={() => setSolverMode("underclued")} />

        <label htmlFor="remainingTiles">Count Tiles Remaining</label>
        <input type="radio" id="tilesRemaining" name="mode" value="remainingTiles" checked={solverMode === "tilesRemaining"} onChange={() => setSolverMode("tilesRemaining")} />
      </div>

      {
        tilesRemaining !== null && <div>Tiles remaining: {tilesRemaining}</div>
      }
      {
        error !== null && <div>{error}</div>
      }
      {
        answer !== null && <AnswerBoard {...answer} />
      }
    </>
  )
}

export default App
