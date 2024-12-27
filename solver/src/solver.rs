use crate::puzzle::{Color, Connection, MinesweeperTile, Puzzle, Rule, Tile};

use cspuz_rs::solver::{Solver, BoolVarArray2D, all};
use cspuz_rs::graph;

fn rotate_pattern(pattern: &[Vec<Color>]) -> Vec<Vec<Color>> {
    let height = pattern.len();
    let width = pattern[0].len();

    let mut rotated = vec![vec![Color::Undecided; height]; width];

    for y in 0..height {
        for x in 0..width {
            rotated[x][height - y - 1] = pattern[y][x];
        }
    }

    rotated
}

fn flip_pattern(pattern: &[Vec<Color>]) -> Vec<Vec<Color>> {
    let height = pattern.len();
    let width = pattern[0].len();

    let mut flipped = vec![vec![Color::Undecided; width]; height];

    for y in 0..height {
        for x in 0..width {
            flipped[y][width - x - 1] = pattern[y][x];
        }
    }

    flipped
}

fn enumerate_patterns(pattern: &[Vec<Color>]) -> Vec<Vec<Vec<Color>>> {
    let mut patterns = vec![];

    let mut p = pattern.to_vec();
    for _ in 0..4 {
        patterns.push(p.clone());
        patterns.push(flip_pattern(&p));
        p = rotate_pattern(&p);
    }

    patterns.sort();
    patterns.dedup();

    patterns
}

struct LogicPadSolver<'a> {
    solver: Solver<'a>,
    height: usize,
    width: usize,
    is_white: BoolVarArray2D,
    is_black: BoolVarArray2D,
}

impl<'a> LogicPadSolver<'a> {
    fn new(height: usize, width: usize) -> LogicPadSolver<'a> {
        let mut solver = Solver::new();
        let is_white = solver.bool_var_2d((height, width));
        let is_black = solver.bool_var_2d((height, width));
        solver.add_answer_key_bool(&is_white);
        solver.add_answer_key_bool(&is_black);

        LogicPadSolver {
            solver,
            height,
            width,
            is_white,
            is_black,
        }
    }

    fn add_tiles(&mut self, tiles: &[Vec<Tile>]) -> Result<(), &'static str> {
        let height = self.height;
        let width = self.width;

        assert_eq!(tiles.len(), height);

        for y in 0..height {
            assert_eq!(tiles[y].len(), width);

            for x in 0..width {
                let tile = &tiles[y][x];

                let b = &self.is_black.at((y, x));
                let w = &self.is_white.at((y, x));

                if !tile.exists {
                    self.solver.add_expr(!b);
                    self.solver.add_expr(!w);

                    continue;
                }

                self.solver.add_expr(b ^ w);

                if tile.fixed {
                    match tile.color {
                        Color::White => {
                            self.solver.add_expr(w);
                        }
                        Color::Black => {
                            self.solver.add_expr(b);
                        }
                        Color::Undecided => {
                            return Err("gray tile is fixed");
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn add_forbidden_pattern(&mut self, pattern: &[Vec<Tile>]) -> Result<(), &'static str> {
        let height = self.height;
        let width = self.width;

        let p_height = pattern.len();
        assert!(p_height > 0);
        let p_width = pattern[0].len();

        for y in 0..p_height {
            assert_eq!(pattern[y].len(), p_width);
        }

        let mut ymin = p_height;
        let mut ymax = 0;
        let mut xmin = p_width;
        let mut xmax = 0;

        for y in 0..p_height {
            for x in 0..p_width {
                if pattern[y][x].color != Color::Undecided {
                    ymin = ymin.min(y);
                    ymax = ymax.max(y);
                    xmin = xmin.min(x);
                    xmax = xmax.max(x);
                }
            }
        }

        if ymin > ymax {
            return Err("empty forbidden pattern");
        }

        let pattern = {
            let mut p = vec![vec![Color::Undecided; xmax - xmin + 1]; ymax - ymin + 1];
            for y in 0..=(ymax - ymin) {
                for x in 0..=(xmax - xmin) {
                    p[y][x] = pattern[y + ymin][x + xmin].color;
                }
            }
            p
        };

        let patterns = enumerate_patterns(&pattern);

        for pat in &patterns {
            let h = pat.len();
            let w = pat[0].len();

            if !(h <= height && w <= width) {
                continue;
            }

            for y in 0..=(height - h) {
                for x in 0..=(width - w) {
                    let mut cond = vec![];

                    for dy in 0..h {
                        for dx in 0..w {
                            match pat[dy][dx] {
                                Color::White => {
                                    cond.push(self.is_white.at((y + dy, x + dx)));
                                }
                                Color::Black => {
                                    cond.push(self.is_black.at((y + dy, x + dx)));
                                }
                                _ => (),
                            }
                        }
                    }

                    self.solver.add_expr(!all(cond));
                }
            }
        }

        Ok(())
    }

    fn add_minesweeper(&mut self, tiles: &[MinesweeperTile]) -> Result<(), &'static str> {
        let height = self.height;
        let width = self.width;

        for tile in tiles {
            let y = tile.y;
            let x = tile.x;
            let num = tile.number;

            let ymin = if y > 0 { y - 1 } else { 0 };
            let ymax = (y + 2).min(height);
            let xmin = if x > 0 { x - 1 } else { 0 };
            let xmax = (x + 2).min(width);

            self.solver.add_expr(self.is_white.at((y, x)).imp(self.is_black.slice((ymin..ymax, xmin..xmax)).count_true().eq(num)));
            self.solver.add_expr(self.is_black.at((y, x)).imp(self.is_white.slice((ymin..ymax, xmin..xmax)).count_true().eq(num)));
        }

        Ok(())
    }

    fn add_connect_all(&mut self, color: Color) -> Result<(), &'static str> {
        match color {
            Color::White => graph::active_vertices_connected_2d(&mut self.solver, &self.is_white),
            Color::Black => graph::active_vertices_connected_2d(&mut self.solver, &self.is_black),
            _ => return Err("connectAll with gray color"),
        }
        Ok(())
    }

    fn add_connections(&mut self, connections: &[Connection]) {
        for conn in connections {
            let y1 = conn.y1;
            let x1 = conn.x1;
            let y2 = conn.y2;
            let x2 = conn.x2;

            self.solver.add_expr(self.is_white.at((y1, x1)).iff(self.is_white.at((y2, x2))));
            self.solver.add_expr(self.is_black.at((y1, x1)).iff(self.is_black.at((y2, x2))));
        }
    }

    fn solve(self) -> Option<Vec<Vec<Option<Color>>>> {
        let model = self.solver.irrefutable_facts()?;

        let is_white = model.get(&self.is_white);
        let is_black = model.get(&self.is_black);
        let mut result = vec![vec![None; self.width]; self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                match (is_white[y][x], is_black[y][x]) {
                    (Some(true), Some(false)) => {
                        result[y][x] = Some(Color::White);
                    }
                    (Some(false), Some(true)) => {
                        result[y][x] = Some(Color::Black);
                    }
                    _ => (),
                }
            }
        }

        Some(result)
    }
}

pub fn solve(puzzle: &Puzzle) -> Result<Option<Vec<Vec<Option<Color>>>>, &'static str> {
    let mut solver = LogicPadSolver::new(puzzle.height, puzzle.width);

    solver.add_tiles(&puzzle.tiles)?;
    solver.add_connections(&puzzle.connections);

    for rule in &puzzle.rules {
        match rule {
            Rule::ConnectAll { color } => solver.add_connect_all(*color)?,
            Rule::ForbiddenPattern { pattern } => {
                solver.add_forbidden_pattern(pattern)?;
            }
            Rule::Minesweeper { tiles } => {
                for tile in tiles {
                    if !puzzle.tiles[tile.y][tile.x].exists {
                        return Err("minesweeper tile on non-existing tile; don't do this");
                    }
                }
                solver.add_minesweeper(tiles)?;
            }
        }
    }

    Ok(solver.solve())
}
