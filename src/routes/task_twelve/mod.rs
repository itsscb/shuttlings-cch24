use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use game::{column::Column, Board, Slot};

pub mod error;
pub mod game;

#[allow(clippy::unused_async)]
pub async fn board(State(board): State<Board>) -> impl IntoResponse {
    board.into_response()
}

#[allow(clippy::unused_async)]
pub async fn reset(State(board): State<Board>) -> impl IntoResponse {
    board.reset();
    board.into_response()
}

#[allow(clippy::unused_async)]
pub async fn place(
    State(board): State<Board>,
    Path((team, column)): Path<(Slot, Column)>,
) -> impl IntoResponse {
    board.insert(column, team).map_err(|err| match err {
        error::GameError::ColumnFull | error::GameError::GameOver => (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            board.to_string(),
        )
            .into_response(),
        e => e.into_response(),
    })?;
    Ok::<_, axum::response::Response>(board.into_response())
}

#[allow(clippy::unused_async)]
pub async fn random_board(State(board): State<Board>) -> impl IntoResponse {
    // let mut random = rand::rngs::StdRng::seed_from_u64(2024);
    let seed = board.get_seed();
    let mut random = seed.lock().unwrap();
    board.random(&mut random);
    board.display().into_response()
}
