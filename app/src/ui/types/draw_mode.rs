use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum DrawMode {
    #[default]
    Ant,
    Nest,
    Food,
}

impl Display for DrawMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
