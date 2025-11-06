const { invoke } = window.__TAURI__.tauri;

let gameState = null;
let gameMode = null;

// DOM 元素
const modeSelection = document.getElementById('mode-selection');
const aiFirstSelection = document.getElementById('ai-first-selection');
const gameBoardContainer = document.getElementById('game-board-container');
const vsPlayerBtn = document.getElementById('vs-player-btn');
const vsAiBtn = document.getElementById('vs-ai-btn');
const playerFirstBtn = document.getElementById('player-first-btn');
const aiFirstBtn = document.getElementById('ai-first-btn');
const restartBtn = document.getElementById('restart-btn');
const currentPlayerEl = document.getElementById('current-player');
const gameStatusEl = document.getElementById('game-status');
const boardEl = document.getElementById('board');

// 事件监听
vsPlayerBtn.addEventListener('click', () => startGame('player', null));
vsAiBtn.addEventListener('click', () => showAiFirstSelection());
playerFirstBtn.addEventListener('click', () => startGame('ai', true));
aiFirstBtn.addEventListener('click', () => startGame('ai', false));
restartBtn.addEventListener('click', () => resetGame());

function showAiFirstSelection() {
    modeSelection.style.display = 'none';
    aiFirstSelection.style.display = 'block';
}

function startGame(mode, playerFirst) {
    gameMode = mode;
    modeSelection.style.display = 'none';
    aiFirstSelection.style.display = 'none';
    gameBoardContainer.style.display = 'block';
    
    invoke('new_game', { 
        gameMode: mode === 'ai' ? 'ai' : 'player',
        playerFirst: playerFirst !== null ? playerFirst : true
    }).then(state => {
        gameState = state;
        renderBoard();
        updateGameInfo();
        
        // 如果AI先手，自动下一手
        if (mode === 'ai' && !playerFirst) {
            setTimeout(() => {
                makeAiMove();
            }, 500);
        }
    });
}

function resetGame() {
    if (gameMode === 'ai') {
        showAiFirstSelection();
    } else {
        modeSelection.style.display = 'block';
    }
    gameBoardContainer.style.display = 'none';
    gameState = null;
}

function renderBoard() {
    boardEl.innerHTML = '';
    
    for (let row = 0; row < 3; row++) {
        for (let col = 0; col < 3; col++) {
            const cell = document.createElement('div');
            cell.className = 'cell';
            cell.dataset.row = row;
            cell.dataset.col = col;
            
            const player = gameState.board[row][col];
            if (player === 'X') {
                cell.textContent = 'X';
                cell.classList.add('x');
            } else if (player === 'O') {
                cell.textContent = 'O';
                cell.classList.add('o');
            }
            
            // 如果游戏结束或位置已被占用，禁用单元格
            if (gameState.game_status !== 'InProgress' || player !== null) {
                cell.classList.add('disabled');
            } else {
                cell.addEventListener('click', () => handleCellClick(row, col));
            }
            
            boardEl.appendChild(cell);
        }
    }
}

function handleCellClick(row, col) {
    if (gameState.game_status !== 'InProgress') {
        return;
    }
    
    // 如果是人机对战且当前是AI的回合，不允许玩家点击
    if (gameState.game_mode === 'VsAI' && gameState.current_player === 'O') {
        return;
    }
    
    invoke('make_move', {
        row: row,
        col: col,
        gameState: gameState
    }).then(newState => {
        gameState = newState;
        renderBoard();
        updateGameInfo();
        
        // 如果是人机对战且游戏未结束，AI自动下棋
        if (gameState.game_mode === 'VsAI' 
            && gameState.game_status === 'InProgress' 
            && gameState.current_player === 'O') {
            setTimeout(() => {
                makeAiMove();
            }, 500);
        }
    }).catch(err => {
        console.error('移动失败:', err);
        alert(err);
    });
}

function makeAiMove() {
    invoke('get_ai_move', {
        gameState: gameState
    }).then(newState => {
        gameState = newState;
        renderBoard();
        updateGameInfo();
    }).catch(err => {
        console.error('AI移动失败:', err);
    });
}

function updateGameInfo() {
    if (gameState.game_status === 'InProgress') {
        const playerName = gameState.current_player === 'X' ? 'X' : 'O';
        const modeText = gameState.game_mode === 'VsAI' && gameState.current_player === 'O' 
            ? 'AI (O)' 
            : playerName === 'X' 
                ? (gameState.game_mode === 'VsAI' ? '玩家 (X)' : '玩家 X')
                : (gameState.game_mode === 'VsAI' ? 'AI (O)' : '玩家 O');
        currentPlayerEl.textContent = `当前玩家: ${modeText}`;
        gameStatusEl.textContent = '';
    } else {
        currentPlayerEl.textContent = '';
        let statusText = '';
        switch (gameState.game_status) {
            case 'XWins':
                statusText = gameState.game_mode === 'VsAI' ? '🎉 玩家获胜！' : '🎉 玩家 X 获胜！';
                break;
            case 'OWins':
                statusText = gameState.game_mode === 'VsAI' ? '🤖 AI获胜！' : '🎉 玩家 O 获胜！';
                break;
            case 'Draw':
                statusText = '平局！';
                break;
        }
        gameStatusEl.textContent = statusText;
        gameStatusEl.style.color = '#e74c3c';
        gameStatusEl.style.fontWeight = 'bold';
    }
}

