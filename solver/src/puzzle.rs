use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    #[serde(rename = "gray")]
    Undecided,
    #[serde(rename = "light")]
    White,
    #[serde(rename = "dark")]
    Black,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tile {
    pub exists: bool,
    pub fixed: bool,
    pub color: Color,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MinesweeperTile {
    pub y: usize,
    pub x: usize,
    pub number: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum Rule {
    #[serde(rename = "connectAll")]
    ConnectAll { color: Color },
    #[serde(rename = "forbiddenPattern")]
    ForbiddenPattern { pattern: Vec<Vec<Tile>> },
    #[serde(rename = "minesweeper")]
    Minesweeper { tiles: Vec<MinesweeperTile> },
}

#[derive(Debug, Clone, Deserialize)]
pub struct Connection {
    pub y1: usize,
    pub x1: usize,
    pub y2: usize,
    pub x2: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Puzzle {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub rules: Vec<Rule>,
    pub connections: Vec<Connection>,
}
