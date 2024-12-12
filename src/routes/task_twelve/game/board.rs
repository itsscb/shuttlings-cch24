use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

pub const DEFAULT_SEED: u64 = 2024;

use axum::response::IntoResponse;
use rand::{rngs::StdRng, Rng, SeedableRng};

use super::{super::error::GameError, column::Column, Slot, EMPTY, WALL};

#[derive(Clone, Debug)]
pub struct Board {
    columns: Arc<Mutex<[[Option<Slot>; 4]; 4]>>,
    seed: Arc<Mutex<StdRng>>,
}

impl Board {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            columns: Arc::new(Mutex::new([[None; 4]; 4])),
            seed: Arc::new(Mutex::new(StdRng::seed_from_u64(DEFAULT_SEED))),
        }
    }

    pub fn display(&self) -> String {
        format!(
            "{}\n",
            self.to_string()
                .split_terminator('\n')
                .take(5)
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    pub fn get_seed(&self) -> Arc<Mutex<StdRng>> {
        self.seed.clone()
    }

    pub fn random(&self, random: &mut StdRng) {
        let mut columns: [[Option<Slot>; 4]; 4] = [[None; 4]; 4];
        // let mut seed = self.seed.lock().unwrap();
        for i in (0..4).rev() {
            (0..4).for_each(|j| {
                let random = random.gen::<bool>();
                let slot = if random {
                    Some(Slot::Cookie)
                } else {
                    Some(Slot::Milk)
                };
                columns[j][i] = slot;
            });
        }
        // for column in &mut columns.iter_mut() {
        //     for slot in column.iter_mut() {
        //         let random = random.gen::<bool>();
        //         if random {
        //             *slot = Some(Slot::Cookie);
        //         } else {
        //             *slot = Some(Slot::Milk);
        //         }
        //     }
        // }
        // drop(seed);

        {
            let mut cols = self.columns.lock().unwrap();
            *cols = columns;
        }
    }

    pub fn is_full(&self) -> bool {
        let columns = self.columns.lock().unwrap();
        columns
            .iter()
            .all(|column| column.iter().all(std::option::Option::is_some))
    }

    pub fn check(&self) -> Option<Slot> {
        let columns = self.columns.lock().unwrap();

        // Check rows and columns
        for i in 0..4 {
            if columns[i][0].is_some()
                && columns[i][0] == columns[i][1]
                && columns[i][0] == columns[i][2]
                && columns[i][0] == columns[i][3]
            {
                return columns[i][0];
            }
            if columns[0][i].is_some()
                && columns[0][i] == columns[1][i]
                && columns[0][i] == columns[2][i]
                && columns[0][i] == columns[3][i]
            {
                return columns[0][i];
            }
        }

        // Check diagonals
        if columns[0][0].is_some()
            && columns[0][0] == columns[1][1]
            && columns[0][0] == columns[2][2]
            && columns[0][0] == columns[3][3]
        {
            return columns[0][0];
        }
        if columns[0][3].is_some()
            && columns[0][3] == columns[1][2]
            && columns[0][3] == columns[2][1]
            && columns[0][3] == columns[3][0]
        {
            return columns[0][3];
        }

        None
    }

    #[allow(dead_code)]
    pub fn reset(&self) {
        {
            let mut columns = self.columns.lock().unwrap();
            for i in 0..4 {
                for j in 0..4 {
                    columns[i][j] = None;
                }
            }
        }
        {
            let mut seed = self.seed.lock().unwrap();
            *seed = StdRng::seed_from_u64(DEFAULT_SEED);
        }
    }

    #[allow(dead_code)]
    pub fn insert(&self, column: Column, slot: Slot) -> Result<(), GameError> {
        if self.check().is_some() {
            return Err(GameError::GameOver);
        }
        {
            let column: usize = column.into();
            let column = column - 1;
            let mut columns = self.columns.lock().unwrap();

            if column > columns.len() - 1 {
                return Err(GameError::InvalidColumn);
            }
            for i in 0..4 {
                if columns[column][i].is_none() {
                    columns[column][i] = Some(slot);
                    return Ok(());
                }
            }
            drop(columns);
        }
        Err(GameError::ColumnFull)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        {
            let columns = self.columns.lock().unwrap();
            for j in (0..4).rev() {
                write!(f, "{WALL}")?;
                for i in 0..4 {
                    match columns[i][j] {
                        Some(slot) => write!(f, "{slot}")?,
                        None => write!(f, "{EMPTY}")?,
                    }
                }
                writeln!(f, "{WALL}")?;
            }
        }
        for _ in 0..6 {
            write!(f, "{WALL}")?;
        }

        writeln!(f)?;
        if let Some(winner) = self.check() {
            writeln!(f, "{winner} wins!")?;
        } else if self.is_full() {
            writeln!(f, "No winner.")?;
        }
        Ok(())
    }
}

impl IntoResponse for Board {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, self.to_string()).into_response()
    }
}
