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

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Orientation {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "up-right")]
    UpRight,
    #[serde(rename = "up-left")]
    UpLeft,
    #[serde(rename = "down-right")]
    DownRight,
    #[serde(rename = "down-left")]
    DownLeft,
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
pub struct AreaNumberTile {
    pub y: usize,
    pub x: usize,
    pub number: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LetterTile {
    pub y: usize,
    pub x: usize,
    pub letter: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DartTile {
    pub y: usize,
    pub x: usize,
    pub orientation: Orientation,
    pub number: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ViewpointTile {
    pub y: usize,
    pub x: usize,
    pub number: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LotusTile {
    pub y: usize,
    pub x: usize,
    pub orientation: Orientation,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GalaxyTile {
    pub y: usize,
    pub x: usize,
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
    #[serde(rename = "number")]
    AreaNumber { tiles: Vec<AreaNumberTile> },
    #[serde(rename = "letter")]
    Letter { tiles: Vec<LetterTile> },
    #[serde(rename = "dart")]
    Dart { tiles: Vec<DartTile> },
    #[serde(rename = "viewpoint")]
    Viewpoint { tiles: Vec<ViewpointTile> },
    #[serde(rename = "lotus")]
    Lotus { tiles: Vec<LotusTile> },
    #[serde(rename = "galaxy")]
    Galaxy { tiles: Vec<GalaxyTile> },
    #[serde(rename = "sameShape")]
    SameShape { color: Color },
    #[serde(rename = "uniqueShape")]
    UniqueShape { color: Color },
    #[serde(rename = "regionArea")]
    RegionArea { color: Color, size: i32 },
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
