use crate::puzzle::{AreaNumberTile, Color, Connection, DartTile, LetterTile, MinesweeperTile, Orientation, Puzzle, Rule, Tile, ViewpointTile};

use cspuz_rs::solver::{all, int_constant, BoolVarArray2D, Solver, count_true, consecutive_prefix_true};
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

    fn add_area_numbers(&mut self, area_numbers: &[AreaNumberTile]) -> Result<(), &'static str> {
        let height = self.height;
        let width = self.width;

        let mut edges = vec![];
        let mut edge_values = vec![];
        let mut sizes = vec![];

        let mut cell_value = vec![vec![None; width]; height];

        for tile in area_numbers {
            if cell_value[tile.y][tile.x].is_some() {
                return Err("duplicate area number");
            }

            cell_value[tile.y][tile.x] = Some(tile.number);
        }

        for y in 0..height {
            for x in 0..width {
                if let Some(n) = cell_value[y][x] {
                    sizes.push(Some(int_constant(n)));
                } else {
                    sizes.push(None);
                }

                if y > 0 {
                    edges.push((y * width + x, (y - 1) * width + x));
                    edge_values.push(!((self.is_white.at((y, x)) & self.is_white.at((y - 1, x))) | (self.is_black.at((y, x)) & self.is_black.at((y - 1, x)))));
                }
                if x > 0 {
                    edges.push((y * width + x, y * width + x - 1));
                    edge_values.push(!((self.is_white.at((y, x)) & self.is_white.at((y, x - 1))) | (self.is_black.at((y, x)) & self.is_black.at((y, x - 1)))));
                }
            }
        }

        self.solver.add_graph_division(&sizes, &edges, &edge_values);

        Ok(())
    }

    fn add_letters(&mut self, letters: &[LetterTile]) -> Result<(), &'static str> {
        let mut letters_sorted = vec![];
        for tile in letters {
            letters_sorted.push((tile.letter.clone(), tile.y, tile.x));
        }
        letters_sorted.sort();

        let mut letter_groups: Vec<Vec<(usize, usize)>> = vec![];
        let mut current_group: Vec<(usize, usize)> = vec![];

        for i in 0..letters_sorted.len() {
            if i > 0 && letters_sorted[i].0 != letters_sorted[i - 1].0 {
                letter_groups.push(current_group);
                current_group = vec![];
            }
            current_group.push((letters_sorted[i].1, letters_sorted[i].2));
        }

        if !current_group.is_empty() {
            letter_groups.push(current_group);
        }

        let height = self.height;
        let width = self.width;
        let group_id = &self.solver.int_var_2d((height, width), -1, letter_groups.len() as i32 - 1);

        self.solver.add_expr((!&self.is_black & !&self.is_white).imp(group_id.eq(-1)));

        let mut adj_pairs = vec![];
        for y in 0..height {
            for x in 0..width {
                if y > 0 {
                    adj_pairs.push(((y, x), (y - 1, x)));
                }
                if x > 0 {
                    adj_pairs.push(((y, x), (y, x - 1)));
                }
            }
        }
        for (p, q) in adj_pairs {
            self.solver.add_expr(
                (group_id.at(p).ne(-1) | group_id.at(p).ne(-1)).imp(
                    (!self.is_black.at(p) & !self.is_white.at(p))
                    | (!self.is_black.at(q) & !self.is_white.at(q))
                    | (group_id.at(p).eq(group_id.at(q)).iff(self.is_black.at(p).iff(self.is_black.at(q)) & self.is_white.at(p).iff(self.is_white.at(q))))
                )
            );
        }

        for i in 0..letter_groups.len() {
            let group = &letter_groups[i];
            for &(y, x) in group {
                self.solver.add_expr(group_id.at((y, x)).eq(i as i32));
            }
            graph::active_vertices_connected_2d(&mut self.solver, group_id.eq(i as i32));
        }

        Ok(())
    }

    fn pointing_cells(&self, y: usize, x: usize, dir: Orientation) -> Vec<(usize, usize)> {
        let mut y = y as i32;
        let mut x = x as i32;

        let (dy, dx) = match dir {
            Orientation::Down => (1, 0),
            Orientation::Right => (0, 1),
            Orientation::Up => (-1, 0),
            Orientation::Left => (0, -1),
            Orientation::DownLeft => (1, -1),
            Orientation::DownRight => (1, 1),
            Orientation::UpLeft => (-1, -1),
            Orientation::UpRight => (-1, 1),
        };

        let mut ret = vec![];

        loop {
            y += dy;
            x += dx;

            if !(0 <= y && y < self.height as i32 && 0 <= x && x < self.width as i32) {
                break;
            }

            ret.push((y as usize, x as usize));
        }

        ret
    }

    fn add_darts(&mut self, darts: &[DartTile]) -> Result<(), &'static str> {
        for dart in darts {
            let y = dart.y;
            let x = dart.x;

            let cells = self.pointing_cells(y, x, dart.orientation);

            let bs = cells.iter().map(|&(y, x)| self.is_black.at((y, x))).collect::<Vec<_>>();
            let ws = cells.iter().map(|&(y, x)| self.is_white.at((y, x))).collect::<Vec<_>>();

            self.solver.add_expr(self.is_black.at((y, x)).imp(count_true(ws).eq(dart.number)));
            self.solver.add_expr(self.is_white.at((y, x)).imp(count_true(bs).eq(dart.number)));
        }

        Ok(())
    }

    fn add_viewpoints(&mut self, viewpoints: &[ViewpointTile]) -> Result<(), &'static str> {
        for tile in viewpoints {
            let y = tile.y;
            let x = tile.x;
            let num = tile.number;

            for a in [&self.is_black, &self.is_white] {
                let mut e = int_constant(1);

                for d in [Orientation::Up, Orientation::Left, Orientation::Down, Orientation::Right] {
                    let cells = self.pointing_cells(y, x, d);
                    let mut cond = vec![];

                    for &q in &cells {
                        cond.push(a.at(q));
                    }

                    e = e + consecutive_prefix_true(&cond);
                }

                self.solver.add_expr(a.at((y, x)).imp(e.eq(num)));
            }
        }

        Ok(())
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
            Rule::AreaNumber { tiles } => {
                for tile in tiles {
                    if !puzzle.tiles[tile.y][tile.x].exists {
                        return Err("area number tile on non-existing tile; don't do this");
                    }
                }
                solver.add_area_numbers(tiles)?;
            }
            Rule::Letter { tiles } => {
                for tile in tiles {
                    if !puzzle.tiles[tile.y][tile.x].exists {
                        return Err("letter tile on non-existing tile; don't do this");
                    }
                }
                solver.add_letters(tiles)?;
            }
            Rule::Dart { tiles } => {
                for tile in tiles {
                    if !puzzle.tiles[tile.y][tile.x].exists {
                        return Err("dart tile on non-existing tile; don't do this");
                    }
                }
                solver.add_darts(tiles)?;
            }
            Rule::Viewpoint { tiles } => {
                for tile in tiles {
                    if !puzzle.tiles[tile.y][tile.x].exists {
                        return Err("viewpoint tile on non-existing tile; don't do this");
                    }
                }
                solver.add_viewpoints(tiles)?;
            }
        }
    }

    Ok(solver.solve())
}
