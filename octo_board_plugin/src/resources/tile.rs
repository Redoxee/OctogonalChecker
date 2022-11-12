
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Octo(usize, usize),
    Quad(usize, usize),
}

impl Tile {
    #[cfg(feature = "debug")]
    #[allow(dead_code)]
    pub fn console_output(&self) -> String {
        format!(
            "{}",
            match self {
                Tile::Octo(x, y) => format!("Octo({x},{y})"),
                Tile::Quad(x, y) => format!("Quad({x},{y})"),
            }
        )
    }
}