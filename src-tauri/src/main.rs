#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod game;

use game::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct GameState {
    board: [[Option<Player>; 3]; 3],
    current_player: Player,
    game_mode: GameMode,
    game_status: GameStatus,
    is_ai_turn: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct MakeMoveRequest {
    row: usize,
    col: usize,
}

#[tauri::command]
fn new_game(game_mode: String, player_first: bool) -> GameState {
    let mode = match game_mode.as_str() {
        "ai" => GameMode::VsAI,
        _ => GameMode::VsPlayer,
    };
    
    let game = Game::new(mode, if mode == GameMode::VsAI { Some(player_first) } else { None });
    
    GameState {
        board: game.board,
        current_player: game.current_player,
        game_mode: game.game_mode,
        game_status: game.check_status(),
        is_ai_turn: mode == GameMode::VsAI && !player_first,
    }
}

#[tauri::command]
fn make_move(row: usize, col: usize, game_state: GameState) -> Result<GameState, String> {
    let mut game = Game {
        board: game_state.board,
        current_player: game_state.current_player,
        game_mode: game_state.game_mode,
    };

    game.make_move(row, col)?;
    
    let mut state = GameState {
        board: game.board,
        current_player: game.current_player,
        game_mode: game.game_mode,
        game_status: game.check_status(),
        is_ai_turn: false,
    };

    // 如果是人机对战且游戏未结束，AI自动下棋
    if state.game_mode == GameMode::VsAI 
        && state.game_status == GameStatus::InProgress 
        && state.current_player == Player::O {
        let ai_move = game.get_best_move();
        if let Some((r, c)) = ai_move {
            game.make_move(r, c)?;
            state.board = game.board;
            state.current_player = game.current_player;
            state.game_status = game.check_status();
        }
    }

    Ok(state)
}

#[tauri::command]
fn get_ai_move(game_state: GameState) -> Result<GameState, String> {
    let mut game = Game {
        board: game_state.board,
        current_player: game_state.current_player,
        game_mode: game_state.game_mode,
    };

    let ai_move = game.get_best_move();
    if let Some((r, c)) = ai_move {
        game.make_move(r, c)?;
    } else {
        return Err("没有可用的移动".to_string());
    }

    Ok(GameState {
        board: game.board,
        current_player: game.current_player,
        game_mode: game.game_mode,
        game_status: game.check_status(),
        is_ai_turn: false,
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![new_game, make_move, get_ai_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

