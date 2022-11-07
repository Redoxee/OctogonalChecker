
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Octo(usize, usize),
    Quad(usize, usize),
}

impl Tile {
    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!(
            "{}",
            match self {
                Tile::Octo(_, _) => "octo",
                Tile::Quad(_, _) => "quad",
            }
        )
    }
}