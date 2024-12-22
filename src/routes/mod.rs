#[cfg(feature = "task12")]
mod task_twelve;
#[cfg(feature = "task12")]
pub use task_twelve::{board, game::Board, place, random_board, reset};

#[cfg(feature = "task16")]
mod task_sixteen;
#[cfg(feature = "task16")]
pub use task_sixteen::{decode, unwrap, wrap};

pub mod task_nineteen;

#[cfg(feature = "task1-9")]
mod hello_bird;

#[cfg(feature = "task1-9")]
mod hello_world;
#[cfg(feature = "task1-9")]
mod minus_one;
#[cfg(feature = "task1-9")]
mod task_five;
#[cfg(feature = "task1-9")]
mod task_nine;
#[cfg(feature = "task1-9")]
mod task_two;

#[cfg(feature = "task1-9")]
pub use hello_bird::hello_bird;
#[cfg(feature = "task1-9")]
pub use hello_world::hello_world;
#[cfg(feature = "task1-9")]
pub use minus_one::minus_one;
#[cfg(feature = "task1-9")]
pub use task_five::manifest;
#[cfg(feature = "task1-9")]
pub use task_two::ipv4_dest;
#[cfg(feature = "task1-9")]
pub use task_two::ipv4_key;
#[cfg(feature = "task1-9")]
pub use task_two::ipv6_dest;
#[cfg(feature = "task1-9")]
pub use task_two::ipv6_key;

#[cfg(feature = "task1-9")]
pub use task_nine::{milk, refill, MilkFactory};
