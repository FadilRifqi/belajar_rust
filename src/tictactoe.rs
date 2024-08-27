use std::io::{self, Write};

pub fn main() {
    println!(" =============================================");
    println!("||               Tic Tac Toe                 ||");
    println!(" =============================================");

    let mut board = vec![' '; 9];
    let mut current_player = 'X';
    let mut next_player = 'O';
    let mut run: bool = true;

    while run {
        print_board(&board);
        let mut input_user = String::new();
        print!("Player {}: Enter your move (1-9): ", current_player);
        io::stdout().flush().unwrap();
        input_user.clear();
        io::stdin().read_line(&mut input_user).unwrap();
        let input_user: usize = match input_user.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 9.");
                continue;
            }
        };

        if input_user < 1 || input_user > 9 {
            println!("Invalid input. Please enter a number between 1 and 9.");
            continue;
        }

        if board[input_user - 1] != ' ' {
            println!("That cell is already taken. Please choose another cell.");
            continue;
        }

        board[input_user - 1] = current_player;

        if check_winner(&board, current_player) {
            print_board(&board);
            println!("Player {} wins!", current_player);
            run = false;
        } else if board.iter().all(|&cell| cell != ' ') {
            print_board(&board);
            println!("It's a draw!");
            run = false;
        }

        std::mem::swap(&mut current_player, &mut next_player);
    }
}

fn print_board(board: &[char]) {
    println!(
        "                 {}  |   {}  |   {}",
        board[0], board[1], board[2]
    );
    println!("             ---------------------");
    println!(
        "                 {}  |   {}  |   {}",
        board[3], board[4], board[5]
    );
    println!("             ---------------------");
    println!(
        "                 {}  |   {}  |   {}",
        board[6], board[7], board[8]
    );
}

fn check_winner(board: &[char], player: char) -> bool {
    let winning_combinations = [
        // Rows
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        // Columns
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        // Diagonals
        [0, 4, 8],
        [2, 4, 6],
    ];

    for &combination in winning_combinations.iter() {
        if combination.iter().all(|&cell| board[cell] == player) {
            return true;
        }
    }

    false
}
