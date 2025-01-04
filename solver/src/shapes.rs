use std::collections::VecDeque;

use cspuz_core::custom_constraints::SimpleCustomConstraint;

fn normalize_shape(shape: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut shape = shape.to_vec();
    shape.sort();
    let min_x = shape.iter().map(|&(x, _)| x).min().unwrap();
    let min_y = shape.iter().map(|&(_, y)| y).min().unwrap();
    for (x, y) in &mut shape {
        *x -= min_x;
        *y -= min_y;
    }
    shape
}

fn rotate_shape(shape: &[(i32, i32)]) -> Vec<(i32, i32)> {
    normalize_shape(&(shape.iter().map(|&(x, y)| (y, -x)).collect::<Vec<_>>()))
}

fn flip_shape(shape: &[(i32, i32)]) -> Vec<(i32, i32)> {
    normalize_shape(&(shape.iter().map(|&(x, y)| (-x, y)).collect::<Vec<_>>()))
}

fn transform_invariant(shape: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut shape = shape.to_vec();
    let mut ret = shape.clone();

    for _ in 0..4 {
        shape = rotate_shape(&shape);
        if shape < ret {
            ret = shape.clone();
        }
        let flip = flip_shape(&shape);
        if flip < ret {
            ret = flip;
        }
    }

    ret
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ShapeCell {
    Inactive,
    Active,
    Undecided,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstraintType {
    AllEqual,
    AllDifferent,
}

pub struct ShapesConstraint {
    height: usize,
    width: usize,
    board: Vec<Vec<ShapeCell>>,
    decision_stack: Vec<(usize, usize)>,
    constraint_type: ConstraintType,
}

impl ShapesConstraint {
    pub fn new(height: usize, width: usize, constraint_type: ConstraintType) -> ShapesConstraint {
        ShapesConstraint {
            height,
            width,
            board: vec![vec![ShapeCell::Undecided; width]; height],
            decision_stack: vec![],
            constraint_type,
        }
    }
}

impl SimpleCustomConstraint for ShapesConstraint {
    fn initialize_sat(&mut self, num_inputs: usize) {
        assert_eq!(num_inputs, self.height * self.width);
    }

    fn notify(&mut self, index: usize, value: bool) {
        let y = index / self.width;
        let x = index % self.width;
        self.board[y][x] = if value {
            ShapeCell::Active
        } else {
            ShapeCell::Inactive
        };
        self.decision_stack.push((y, x));
    }

    fn find_inconsistency(&mut self) -> Option<Vec<(usize, bool)>> {
        let height = self.height;
        let width = self.width;

        let mut block_id = vec![vec![!0; width]; height];
        let mut last_id = 0;
        let mut queue = VecDeque::new();
        let mut closed_blocks = vec![];

        for y in 0..height {
            for x in 0..width {
                if self.board[y][x] != ShapeCell::Active {
                    continue;
                }

                if block_id[y][x] != !0 {
                    continue;
                }

                assert!(queue.is_empty());
                queue.push_back((y, x));
                block_id[y][x] = last_id;
                let mut is_closed = true;
                let mut block = vec![];

                while let Some((y, x)) = queue.pop_front() {
                    block.push((y, x));
                    for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let ny = y as i32 + dy;
                        let nx = x as i32 + dx;

                        if 0 <= ny && ny < height as i32 && 0 <= nx && nx < width as i32 {
                            let ny = ny as usize;
                            let nx = nx as usize;
                            if self.board[ny][nx] == ShapeCell::Undecided {
                                is_closed = false;
                            }
                            if self.board[ny][nx] == ShapeCell::Active && block_id[ny][nx] == !0 {
                                block_id[ny][nx] = last_id;
                                queue.push_back((ny, nx));
                            }
                        }
                    }
                }

                if is_closed {
                    let block_i32 = block.iter().map(|&(y, x)| (y as i32, x as i32)).collect::<Vec<_>>();
                    let block_invariant = transform_invariant(&block_i32);
                    closed_blocks.push((block_invariant, block));
                }

                last_id += 1;
            }
        }

        if closed_blocks.len() == 0 {
            return None;
        }

        let mut inconsistent_pair: Option<(usize, usize)> = None;

        match self.constraint_type {
            ConstraintType::AllDifferent => {
                for i in 1..closed_blocks.len() {
                    for j in 0..i {
                        if closed_blocks[i].0 == closed_blocks[j].0 {
                            inconsistent_pair = Some((i, j));
                            break;
                        }
                    }
                    if inconsistent_pair.is_some() {
                        break;
                    }
                }
            }
            ConstraintType::AllEqual => {
                for i in 1..closed_blocks.len() {
                    if closed_blocks[i].0 != closed_blocks[0].0 {
                        inconsistent_pair = Some((i, 0));
                        break;
                    }
                }
            }
        }

        if let Some((p, q)) = inconsistent_pair {
            let mut ret = vec![];

            for b in [p, q] {
                for &(y, x) in &closed_blocks[b].1 {
                    let index = y * width + x;
                    ret.push((index, true));

                    for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let ny = y as i32 + dy;
                        let nx = x as i32 + dx;

                        if 0 <= ny && ny < height as i32 && 0 <= nx && nx < width as i32 {
                            let ny = ny as usize;
                            let nx = nx as usize;
                            if self.board[ny][nx] == ShapeCell::Inactive {
                                ret.push((ny * width + nx, false));
                            }
                        }
                    }
                }
            }

            Some(ret)
        } else {
            None
        }
    }

    fn undo(&mut self) {
        let (y, x) = self.decision_stack.pop().unwrap();
        self.board[y][x] = ShapeCell::Undecided;
    }
}
