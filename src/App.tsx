import { useState } from 'react';

import { solve, loadSolver } from './solverBridge';
import { urlToPuzzle, puzzleToJson } from './jsonify';
import { AnswerBoard } from './board';

type AnswerData = {
  board: readonly (readonly TileData[])[];
  answer: ("light" | "dark" | null)[][];
};

function App() {
  const [url, setUrl] = useState<string>("");
  const [answer, setAnswer] = useState<AnswerData | null>(null);
  const [error, setError] = useState<string | null>(null);

  const runSolver = async () => {
    const puzzle = await urlToPuzzle(url);

    try {
      const json = puzzleToJson(puzzle);
    } catch (e: any) {
      setError(e.message);
      setAnswer(null);
      return;
    }
    await loadSolver();

    const result = JSON.parse(solve(json));

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
    setAnswer({
      board: puzzle.grid.tiles,
      answer: result,
    });
  };

  return (
    <>
      <div>
        <input type="text" value={url} onChange={e => setUrl(e.target.value)} />
        <input type="button" value="Solve" onClick={runSolver} />
      </div>
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
