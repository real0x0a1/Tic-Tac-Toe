use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to Tic-Tac-Toe!");
    println!("1. 2-Player");
    println!("2. 1-Player (with AI)");
    println!("Enter your choice:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u8 = choice.trim().parse().expect("Invalid input");

    let mut board = ['.'; 9];
    let mut current_player = 'X';

    match choice {
        1 => {
            println!("2-Player mode selected.");
            print_board(&board);

            loop {
                let mut input = String::new();
                println!("Player {}'s turn. Enter a position (1-9):", current_player);
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let position: usize = input.trim().parse().expect("Invalid input");

                if board[position - 1] != '.' {
                    println!("Invalid move. Try again.");
                    continue;
                }

                board[position - 1] = current_player;
                print_board(&board);

                if check_win(&board, current_player) {
                    println!("Player {} wins!", current_player);
                    break;
                }

                if check_draw(&board) {
                    println!("It's a draw!");
                    break;
                }

                current_player = if current_player == 'X' { 'O' } else { 'X' };
            }
        }
        2 => {
            println!("1-Player mode selected.");
            print_board(&board);

            loop {
                let mut input = String::new();
                println!("Player {}'s turn. Enter a position (1-9):", current_player);
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let position: usize = input.trim().parse().expect("Invalid input");

                if board[position - 1] != '.' {
                    println!("Invalid move. Try again.");
                    continue;
                }

                board[position - 1] = current_player;
                print_board(&board);

                if check_win(&board, current_player) {
                    println!("Player {} wins!", current_player);
                    break;
                }

                if check_draw(&board) {
                    println!("It's a draw!");
                    break;
                }

                current_player = if current_player == 'X' { 'O' } else { 'X' };

                if current_player == 'O' {
                    ai_move(&mut board);
                    print_board(&board);

                    if check_win(&board, current_player) {
                        println!("AI wins!");
                        break;
                    }

                    if check_draw(&board) {
                        println!("It's a draw!");
                        break;
                    }
                }
            }
        }
        _ => println!("Invalid choice."),
    }
}

fn print_board(board: &[char; 9]) {
    println!("\n");
    println!("{}|{}|{}", board[0], board[1], board[2]);
    println!("-+-+-");
    println!("{}|{}|{}", board[3], board[4], board[5]);
    println!("-+-+-");
    println!("{}|{}|{}", board[6], board[7], board[8]);
}

fn check_win(board: &[char; 9], player: char) -> bool {
    let win_patterns = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
        [0, 3, 6], [1, 4, 7], [2, 5, 8], // columns
        [0, 4, 8], [2, 4, 6],            // diagonals
    ];

    for pattern in &win_patterns {
        if board[pattern[0]] == player
            && board[pattern[1]] == player
            && board[pattern[2]] == player
        {
            return true;
        }
    }

    false
}

fn check_draw(board: &[char; 9]) -> bool {
    board.iter().all(|&c| c != '.')
}

fn ai_move(board: &mut [char; 9]) {
    let mut rng = rand::thread_rng();
    let mut position = rng.gen_range(0..9);

    while board[position] != '.' {
        position = rng.gen_range(0..9);
    }

    board[position] = 'O';
}