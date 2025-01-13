use crate::puzzle::{AreaNumberTile, Color, Connection, DartTile, GalaxyTile, LetterTile, LotusTile, MinesweeperTile, Orientation, Puzzle, Rule, Tile, ViewpointTile};

use cspuz_rs::solver::{all, int_constant, BoolVarArray2D, Solver, count_true, consecutive_prefix_true};
use cspuz_rs::graph;
use crate::shapes::{ConstraintType, ShapesConstraint};

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
    off_by: Option<i32>,
    is_white: BoolVarArray2D,
    is_black: BoolVarArray2D,
}

impl<'a> LogicPadSolver<'a> {
    fn new(height: usize, width: usize, off_by: Option<i32>) -> LogicPadSolver<'a> {
        let mut solver = Solver::new();
        let is_white = solver.bool_var_2d((height, width));
        let is_black = solver.bool_var_2d((height, width));
        solver.add_answer_key_bool(&is_white);
        solver.add_answer_key_bool(&is_black);

        LogicPadSolver {
            solver,
            height,
            width,
            off_by,
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

            if let Some(off_by) = self.off_by {
                self.solver.add_expr(self.is_white.at((y, x)).imp(self.is_black.slice((ymin..ymax, xmin..xmax)).count_true().eq(num + off_by) | self.is_black.slice((ymin..ymax, xmin..xmax)).count_true().eq(num - off_by)));
                self.solver.add_expr(self.is_black.at((y, x)).imp(self.is_white.slice((ymin..ymax, xmin..xmax)).count_true().eq(num + off_by) | self.is_white.slice((ymin..ymax, xmin..xmax)).count_true().eq(num - off_by)));
            } else {
                self.solver.add_expr(self.is_white.at((y, x)).imp(self.is_black.slice((ymin..ymax, xmin..xmax)).count_true().eq(num)));
                self.solver.add_expr(self.is_black.at((y, x)).imp(self.is_white.slice((ymin..ymax, xmin..xmax)).count_true().eq(num)));
            }
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

    fn add_connect_all_both_color(&mut self) {
        let height = self.height;
        let width = self.width;

        for y in 0..(height - 1) {
            for x in 0..(width - 1) {
                self.solver.add_expr(!(self.is_white.at((y, x)) & self.is_black.at((y, x + 1)) & self.is_black.at((y + 1, x)) & self.is_white.at((y + 1, x + 1))));
                self.solver.add_expr(!(self.is_black.at((y, x)) & self.is_white.at((y, x + 1)) & self.is_white.at((y + 1, x)) & self.is_black.at((y + 1, x + 1))));
            }
        }

        let mut circ = vec![];
        for x in 0..width {
            circ.push((0, x));
        }
        for y in 1..(height - 1) {
            circ.push((y, width - 1));
        }
        for x in (0..width - 1).rev() {
            circ.push((height - 1, x));
        }
        for y in (1..(height - 1)).rev() {
            circ.push((y, 0));
        }

        let mut boundaries = vec![];
        for i in 0..circ.len() {
            let p = circ[i];
            let q = circ[(i + 1) % circ.len()];

            boundaries.push((self.is_white.at(p) & self.is_black.at(q)) | (self.is_black.at(p) & self.is_white.at(q)));
        }

        self.solver.add_expr(count_true(boundaries).le(2));
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

    fn add_area_numbers(&mut self,
        area_numbers: Option<&[AreaNumberTile]>,
        size_light: Option<i32>,
        size_dark: Option<i32>,
    ) -> Result<(), &'static str> {
        if area_numbers.is_none() && size_light.is_none() && size_dark.is_none() {
            return Ok(());
        }

        let height = self.height;
        let width = self.width;

        let mut edges = vec![];
        let mut edge_values = vec![];
        let mut sizes = vec![];

        let mut cell_value = vec![vec![None; width]; height];

        if let Some(area_numbers) = area_numbers {
            for tile in area_numbers {
                if cell_value[tile.y][tile.x].is_some() {
                    return Err("duplicate area number");
                }

                cell_value[tile.y][tile.x] = Some(tile.number);
            }
        }

        for y in 0..height {
            for x in 0..width {
                if size_light.is_some() || size_dark.is_some() {
                    let sz = self.solver.int_var(1, (height * width) as i32);

                    if let Some(n) = size_light {
                        self.solver.add_expr(self.is_white.at((y, x)).imp(sz.eq(n)));
                    }
                    if let Some(n) = size_dark {
                        self.solver.add_expr(self.is_black.at((y, x)).imp(sz.eq(n)));
                    }
                    if let Some(n) = cell_value[y][x] {
                        if let Some(off_by) = self.off_by {
                            self.solver.add_expr(sz.eq(n + off_by) | sz.eq(n - off_by));
                        } else {
                            self.solver.add_expr(sz.eq(n));
                        }
                    }

                    sizes.push(Some(sz.expr()));
                } else {
                    if let Some(n) = cell_value[y][x] {
                        if let Some(off_by) = self.off_by {
                            if n - off_by > 0 {
                                let sz = self.solver.int_var_from_domain(vec![n - off_by, n + off_by]);
                                sizes.push(Some(sz.expr()));
                            } else {
                                sizes.push(Some(int_constant(n + off_by)));
                            }
                        } else {
                            sizes.push(Some(int_constant(n)));
                        }
                    } else {
                        sizes.push(None);
                    }
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

            if let Some(off_by) = self.off_by {
                self.solver.add_expr(self.is_black.at((y, x)).imp(count_true(&ws).eq(dart.number + off_by) | count_true(&ws).eq(dart.number - off_by)));
                self.solver.add_expr(self.is_white.at((y, x)).imp(count_true(&bs).eq(dart.number + off_by) | count_true(&bs).eq(dart.number - off_by)));
            } else {
                self.solver.add_expr(self.is_black.at((y, x)).imp(count_true(&ws).eq(dart.number)));
                self.solver.add_expr(self.is_white.at((y, x)).imp(count_true(&bs).eq(dart.number)));
            }
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

                if let Some(off_by) = self.off_by {
                    self.solver.add_expr(a.at((y, x)).imp(e.eq(num + off_by) | e.eq(num - off_by)));
                } else {
                    self.solver.add_expr(a.at((y, x)).imp(e.eq(num)));
                }
            }
        }

        Ok(())
    }

    fn add_lotus_or_galaxy(&mut self, y: usize, x: usize, sy: usize, sx: usize, ori: Option<Orientation>) -> Result<(), &'static str> {
        let height = self.height;
        let width = self.width;

        if !(y < height && x < width) {
            return Err("lotus out of bounds");
        }

        let block_cells = &self.solver.bool_var_2d((height, width));
        graph::active_vertices_connected_2d(&mut self.solver, block_cells);
        self.solver.add_expr(block_cells.at((y, x)));

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
                (self.is_black.at(p) & self.is_black.at(q)).imp(block_cells.at(p).iff(block_cells.at(q)))
            );
            self.solver.add_expr(
                (self.is_white.at(p) & self.is_white.at(q)).imp(block_cells.at(p).iff(block_cells.at(q)))
            );
            self.solver.add_expr(
                (self.is_black.at(p) ^ self.is_black.at(q)).imp(!block_cells.at(p) | !block_cells.at(q))
            );
            self.solver.add_expr(
                (self.is_white.at(p) ^ self.is_white.at(q)).imp(!block_cells.at(p) | !block_cells.at(q))
            );
        }

        for y in 0..(height as i32) {
            for x in 0..(width as i32) {
                let (y2, x2) = match ori {
                    Some(Orientation::Down) | Some(Orientation::Up) => {
                        (y, sx as i32 - x)
                    }
                    Some(Orientation::Left) | Some(Orientation::Right) => {
                        (sy as i32 - y, x)
                    }
                    Some(Orientation::DownLeft) | Some(Orientation::UpRight) => {
                        ((sx + sy) as i32 / 2 - x, (sx + sy) as i32 / 2 - y)
                    }
                    Some(Orientation::DownRight) | Some(Orientation::UpLeft) => {
                        ((sy as i32 - sx as i32) / 2 + x, (sx as i32 - sy as i32) / 2 + y)
                    }
                    None => {
                        (sy as i32 - y, sx as i32 - x)
                    }
                };

                if !(0 <= y2 && y2 < height as i32 && 0 <= x2 && x2 < width as i32) {
                    self.solver.add_expr(!block_cells.at((y as usize, x as usize)));
                    continue;
                }

                if (y, x) < (y2, x2) {
                    self.solver.add_expr(block_cells.at((y as usize, x as usize)).iff(block_cells.at((y2 as usize, x2 as usize))));
                }
            }
        }

        Ok(())
    }

    fn add_lotuses(&mut self, lotuses: &[LotusTile]) -> Result<(), &'static str> {
        for tile in lotuses {
            match tile.orientation {
                Orientation::Down | Orientation::Up => {
                    if tile.y % 2 == 0 {
                        self.add_lotus_or_galaxy(tile.y / 2, tile.x / 2, tile.y, tile.x, Some(tile.orientation))?;
                    } else {
                        return Err("lotus on invalid position");
                    }
                }
                Orientation::Left | Orientation::Right => {
                    if tile.x % 2 == 0 {
                        self.add_lotus_or_galaxy(tile.y / 2, tile.x / 2, tile.y, tile.x, Some(tile.orientation))?;
                    } else {
                        return Err("lotus on invalid position");
                    }
                }
                Orientation::DownLeft | Orientation::UpRight => {
                    if tile.x % 2 == 0 && tile.y % 2 == 0 {
                        self.add_lotus_or_galaxy(tile.y / 2, tile.x / 2, tile.y, tile.x, Some(tile.orientation))?;
                    } else {
                        return Err("lotus on invalid position");
                    }
                }
                Orientation::DownRight | Orientation::UpLeft => {
                    if tile.x % 2 == 0 && tile.y % 2 == 0 {
                        self.add_lotus_or_galaxy(tile.y / 2, tile.x / 2, tile.y, tile.x, Some(tile.orientation))?;
                    } else {
                        return Err("lotus on invalid position");
                    }
                }
            }
        }

        Ok(())
    }

    fn add_galaxies(&mut self, galaxies: &[GalaxyTile]) -> Result<(), &'static str> {
        for tile in galaxies {
            if tile.x % 2 == 1 && tile.y % 2 == 1 {
                return Err("galaxies on corners may cause unexpected behavior");
            }
            self.add_lotus_or_galaxy(tile.y / 2, tile.x / 2, tile.y, tile.x, None)?;
        }

        Ok(())
    }

    fn add_same_shape(&mut self, color: Color) {
        self.solver.add_custom_constraint(
            Box::new(ShapesConstraint::new(self.height, self.width, ConstraintType::AllEqual)),
            match color {
                Color::White => &self.is_white,
                Color::Black => &self.is_black,
                _ => panic!(),
            },
        );
    }

    fn add_unique_shape(&mut self, color: Color) {
        self.solver.add_custom_constraint(
            Box::new(ShapesConstraint::new(self.height, self.width, ConstraintType::AllDifferent)),
            match color {
                Color::White => &self.is_white,
                Color::Black => &self.is_black,
                _ => panic!(),
            },
        );
    }

    fn add_cell_count(&mut self, color: Color, count: i32) {
        match color {
            Color::White => self.solver.add_expr(self.is_white.count_true().eq(count)),
            Color::Black => self.solver.add_expr(self.is_black.count_true().eq(count)),
            _ => panic!(),
        }
    }

    fn solve(self, underclued: bool) -> Option<Vec<Vec<Option<Color>>>> {
        if underclued {
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
        } else {
            let mut solver = self.solver;
            let model = solver.solve()?;

            let is_white = model.get(&self.is_white);
            let is_black = model.get(&self.is_black);
            let mut result = vec![vec![None; self.width]; self.height];
            for y in 0..self.height {
                for x in 0..self.width {
                    match (is_white[y][x], is_black[y][x]) {
                        (true, false) => {
                            result[y][x] = Some(Color::White);
                        }
                        (false, true) => {
                            result[y][x] = Some(Color::Black);
                        }
                        _ => (),
                    }
                }
            }

            Some(result)
        }
    }
}

pub fn solve(puzzle: &Puzzle, underclued: bool) -> Result<Option<Vec<Vec<Option<Color>>>>, &'static str> {
    let mut off_by = None;

    for rule in &puzzle.rules {
        match rule {
            Rule::OffByX { number } => {
                if off_by.is_some() {
                    return Err("multiple offByX rules");
                }
                if *number <= 0 {
                    return Err("offByX with non-positive number");
                }
                off_by = Some(*number);
            }
            _ => (),
        }
    }

    let mut solver = LogicPadSolver::new(puzzle.height, puzzle.width, off_by);

    solver.add_tiles(&puzzle.tiles)?;
    solver.add_connections(&puzzle.connections);

    let mut has_connect_all_white = false;
    let mut has_connect_all_black = false;

    for rule in &puzzle.rules {
        match rule {
            Rule::ConnectAll { color } => {
                match *color {
                    Color::White => has_connect_all_white = true,
                    Color::Black => has_connect_all_black = true,
                    _ => return Err("connectAll with gray color"),
                }
                solver.add_connect_all(*color)?;
            }
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
            Rule::AreaNumber { tiles: _ } => (),
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
            Rule::Lotus { tiles } => {
                solver.add_lotuses(tiles)?;
            }
            Rule::Galaxy { tiles } => {
                solver.add_galaxies(tiles)?;
            }
            Rule::SameShape { color } => {
                solver.add_same_shape(*color);
            }
            Rule::UniqueShape { color } => {
                solver.add_unique_shape(*color);
            }
            Rule::RegionArea { color: _, size: _ } => (),
            Rule::CellCount { color, count } => {
                solver.add_cell_count(*color, *count);
            }
            Rule::OffByX { number: _ } => (),
        }
    }

    if has_connect_all_white && has_connect_all_black {
        solver.add_connect_all_both_color();
    }

    // Area size constraints
    let mut area_number: Option<&[AreaNumberTile]> = None;
    let mut size_light = None;
    let mut size_dark = None;

    for rule in &puzzle.rules {
        match rule {
            Rule::AreaNumber { tiles } => {
                if area_number.is_some() {
                    return Err("multiple area number rules");
                }
                area_number = Some(tiles);
            }
            Rule::RegionArea { color, size } => {
                match *color {
                    Color::White => {
                        if size_light.is_some() {
                            return Err("multiple light area size rules");
                        }
                        size_light = Some(*size);
                    }
                    Color::Black => {
                        if size_dark.is_some() {
                            return Err("multiple dark area size rules");
                        }
                        size_dark = Some(*size);
                    }
                    _ => panic!(),
                }
            }
            _ => (),
        }
    }

    solver.add_area_numbers(area_number, size_light, size_dark)?;

    Ok(solver.solve(underclued))
}
