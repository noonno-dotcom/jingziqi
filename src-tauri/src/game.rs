use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    VsPlayer,
    VsAI,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameStatus {
    InProgress,
    XWins,
    OWins,
    Draw,
}

pub struct Game {
    pub board: [[Option<Player>; 3]; 3],
    pub current_player: Player,
    pub game_mode: GameMode,
}

impl Game {
    pub fn new(game_mode: GameMode, player_first: Option<bool>) -> Self {
        let current_player = match (game_mode, player_first) {
            (GameMode::VsAI, Some(true)) => Player::X,  // 玩家先手
            (GameMode::VsAI, Some(false)) => Player::O, // AI先手
            _ => Player::X, // 玩家对战默认X先手
        };

        Self {
            board: [[None; 3]; 3],
            current_player,
            game_mode,
        }
    }

    pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), String> {
        if row >= 3 || col >= 3 {
            return Err("位置超出范围".to_string());
        }

        if self.board[row][col].is_some() {
            return Err("该位置已被占用".to_string());
        }

        self.board[row][col] = Some(self.current_player);
        self.current_player = self.current_player.other();
        Ok(())
    }

    pub fn check_status(&self) -> GameStatus {
        // 检查行
        for row in 0..3 {
            if let Some(player) = self.board[row][0] {
                if self.board[row][1] == Some(player) && self.board[row][2] == Some(player) {
                    return if player == Player::X { GameStatus::XWins } else { GameStatus::OWins };
                }
            }
        }

        // 检查列
        for col in 0..3 {
            if let Some(player) = self.board[0][col] {
                if self.board[1][col] == Some(player) && self.board[2][col] == Some(player) {
                    return if player == Player::X { GameStatus::XWins } else { GameStatus::OWins };
                }
            }
        }

        // 检查对角线
        if let Some(player) = self.board[0][0] {
            if self.board[1][1] == Some(player) && self.board[2][2] == Some(player) {
                return if player == Player::X { GameStatus::XWins } else { GameStatus::OWins };
            }
        }

        if let Some(player) = self.board[0][2] {
            if self.board[1][1] == Some(player) && self.board[2][0] == Some(player) {
                return if player == Player::X { GameStatus::XWins } else { GameStatus::OWins };
            }
        }

        // 检查是否平局
        let is_full = self.board.iter().all(|row| row.iter().all(|cell| cell.is_some()));
        if is_full {
            return GameStatus::Draw;
        }

        GameStatus::InProgress
    }

    pub fn get_best_move(&self) -> Option<(usize, usize)> {
        // 使用极小极大算法找到最佳移动
        // AI总是O，所以应该最大化O的收益
        let mut best_score = i32::MIN;
        let mut best_move = None;

        // AI是O，所以从O的角度来看
        // O获胜 = 高分，X获胜 = 低分
        let is_ai_turn = self.current_player == Player::O;

        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col].is_none() {
                    let mut test_game = Game {
                        board: self.board,
                        current_player: self.current_player,
                        game_mode: self.game_mode,
                    };
                    
                    if test_game.make_move(row, col).is_ok() {
                        // 从AI（O）的角度评估，所以is_maximizing = true
                        let score = minimax(&test_game, !is_ai_turn, 0);
                        if score > best_score {
                            best_score = score;
                            best_move = Some((row, col));
                        }
                    }
                }
            }
        }

        best_move
    }
}

fn minimax(game: &Game, is_maximizing: bool, depth: i32) -> i32 {
    // 从AI（O）的角度评分
    match game.check_status() {
        GameStatus::XWins => return -10 + depth,  // X获胜对AI不利
        GameStatus::OWins => return 10 - depth,   // O获胜对AI有利
        GameStatus::Draw => return 0,
        GameStatus::InProgress => {}
    }

    if is_maximizing {
        // 最大化玩家（O/AI）的回合
        let mut best_score = i32::MIN;
        for row in 0..3 {
            for col in 0..3 {
                if game.board[row][col].is_none() {
                    let mut test_game = Game {
                        board: game.board,
                        current_player: game.current_player,
                        game_mode: game.game_mode,
                    };
                    if test_game.make_move(row, col).is_ok() {
                        let score = minimax(&test_game, false, depth + 1);
                        best_score = best_score.max(score);
                    }
                }
            }
        }
        best_score
    } else {
        // 最小化玩家（X/人类）的回合
        let mut best_score = i32::MAX;
        for row in 0..3 {
            for col in 0..3 {
                if game.board[row][col].is_none() {
                    let mut test_game = Game {
                        board: game.board,
                        current_player: game.current_player,
                        game_mode: game.game_mode,
                    };
                    if test_game.make_move(row, col).is_ok() {
                        let score = minimax(&test_game, true, depth + 1);
                        best_score = best_score.min(score);
                    }
                }
            }
        }
        best_score
    }
}

