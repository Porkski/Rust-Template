use std::io;

pub fn run() {
    let mut board = [['-'; 3]; 3];
    let mut turn = 'X';
    let mut count = 0;

    while count < 9 {
        // Print board
        for row in 0..3 {
            for col in 0..3 {
                print!("{} ", board[col][row]);
            }
            println!();
        }

        // Get input
        println!("Player {} what x coordinate?", turn);
        let x: usize = read_input();
        println!("Player {} what y coordinate?", turn);
        let y: usize = read_input();

        // Check valid move
        if x < 3 && y < 3 && board[x][y] == '-' {
            board[x][y] = turn;
            
            // Check win
            if check_win(&board, turn) {
                println!("{} wins!", turn);
                return;
            }
            
            turn = if turn == 'X' { 'O' } else { 'X' };
            count += 1;
        } else {
            println!("Invalid move.");
        }
    }
    
    println!("It's a tie!");
}

fn read_input() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or(99)
}

fn check_win(board: &[[char; 3]; 3], player: char) -> bool {
    // Rows
    for row in 0..3 {
        if board[row][0] == player && board[row][1] == player && board[row][2] == player {
            return true;
        }
    }
    // Columns  
    for col in 0..3 {
        if board[0][col] == player && board[1][col] == player && board[2][col] == player {
            return true;
        }
    }
    // Diagonals
    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }
    if board[2][0] == player && board[1][1] == player && board[0][2] == player {
        return true;
    }
    false
}