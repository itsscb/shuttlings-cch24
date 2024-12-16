pub mod board;
pub mod column;
pub mod slot;

pub use board::Board;
pub use slot::Slot;

const EMPTY: &str = "⬛";
const COOKIE: &str = "🍪";
const MILK: &str = "🥛";
const WALL: &str = "⬜";
