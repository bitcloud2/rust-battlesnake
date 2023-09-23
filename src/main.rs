use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;

mod logic;

// API and Response Objects
// See https://docs.battlesnake.com/api

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    id: String,
    ruleset: HashMap<String, Value>,
    timeout: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Board {
    height: u32,
    width: u32,
    food: Vec<Coord>,
    snakes: Vec<Battlesnake>,
    hazards: Vec<Coord>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Battlesnake {
    id: String,
    name: String,
    health: u32,
    body: Vec<Coord>,
    head: Coord,
    length: u32,
    latency: String,
    shout: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Coord {
    x: u32,
    y: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameState {
    game: Game,
    turn: u32,
    board: Board,
    you: Battlesnake,
}
#[tokio::main]
async fn main() {
    // We default to 'info' level logging. But if the `RUST_LOG` environment variable is set,
    // we keep that value instead.
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `handle_index`
        .route("/", get(handle_index))
        // `POST /start` goes to `handle_start`
        .route("/start", post(handle_start))
        .route("/move", post(handle_move))
        .route("/end", post(handle_end));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Starting BattleSnake Server...");
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_index() -> Json<Value> {
    Json(logic::info())
}

async fn handle_start(Json(start_req): Json<GameState>) -> StatusCode {
    logic::start(
        &start_req.game,
        &start_req.turn,
        &start_req.board,
        &start_req.you,
    );

    StatusCode::OK
}

async fn handle_move(Json(move_req): Json<GameState>) -> Json<Value> {
    let response = logic::get_move(
        &move_req.game,
        &move_req.turn,
        &move_req.board,
        &move_req.you,
    );

    Json(response)
}

async fn handle_end(Json(end_req): Json<GameState>) -> StatusCode {
    logic::end(&end_req.game, &end_req.turn, &end_req.board, &end_req.you);

    StatusCode::OK
}