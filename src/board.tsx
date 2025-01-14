import { TileData } from '@logic-pad/core';

export type AnswerBoardProps = {
  board: readonly (readonly TileData[])[];
  answer: ("light" | "dark" | null)[][];
}

export const AnswerBoard = (props: AnswerBoardProps) => {
  const items = [];

  const { board, answer} = props;
  const height = board.length;
  const width = board[0].length;

  const cellSize = 30;
  const offset = 10;

  items.push(<rect x={0} y={0} width={width * cellSize + 2 * offset} height={height * cellSize + 2 * offset} fill="#aaaabb" />);

  for (let y = 0; y < height; ++y) {
    for (let x = 0; x < width; ++x) {
      let color = "#ccccdd";

      if (!board[y][x].exists) {
        color = "#aaaacc";
      } else if (answer[y][x] === "dark") {
        color = "#000000";
      } else if (answer[y][x] === "light") {
        color = "#ffffff";
      } else {
        color = "#ccccdd";
      }

      items.push(<rect x={x * cellSize + offset + 2} y={y * cellSize + offset + 2} width={cellSize - 4} height={cellSize - 4} fill={color} />);
    }
  }

  return <svg width={width * cellSize + 2 * offset} height={height * cellSize + 2 * offset}>{items}</svg>;
};
