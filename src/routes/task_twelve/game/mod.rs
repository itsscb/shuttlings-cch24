pub mod board;
pub mod column;
pub mod slot;

pub use board::Board;
pub use slot::Slot;

const EMPTY: &str = "⬛";
const COOKIE: &str = "🍪";
const MILK: &str = "🥛";
const WALL: &str = "⬜";

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_board() {
//         let board = Board::new();
//         println!("{board}");

//         for _ in 0..4 {
//             assert!(board.insert(0, Slot::Milk).is_ok());
//         }

//         println!("{board}");
//     }
// }
