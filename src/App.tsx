import { useState } from 'react';

import { urlToPuzzle, puzzleToJson } from './jsonify';
import { AnswerBoard } from './board';

type AnswerData = {
  board: readonly (readonly TileData[])[];
  answer: ("light" | "dark" | null)[][];
};

let workerInstance: any = null;

function App() {
  const [url, setUrl] = useState<string>("");
  const [answer, setAnswer] = useState<AnswerData | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isRunning, setIsRunning] = useState<boolean>(false);
  const [underclued, setUnderclued] = useState<boolean>(false);

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

    setIsRunning(true);
    const result = JSON.parse(await workerInstance!.solve(json, underclued));
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
    setAnswer({
      board: puzzle.grid.tiles,
      answer: result,
    });
  };

  return (
    <>
      <div>
        URL: <input type="text" value={url} onChange={e => setUrl(e.target.value)} size={40} />
        <input type="button" value="Solve" onClick={runSolver} disabled={isRunning} />

        <label htmlFor="underclued">Underclued</label>
        <input type="checkbox" id="underclued" checked={underclued} onChange={e => setUnderclued(e.target.checked)} />
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
