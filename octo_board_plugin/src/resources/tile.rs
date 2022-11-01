
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Octo,
    Quad,
}

impl Tile {
    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!(
            "{}",
            match self {
                Tile::Octo => "octo",
                Tile::Quad => "quad",
            }
        )
    }
}