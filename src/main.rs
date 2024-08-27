use std::io::{self, Write};
mod blockchain;
mod calculator;
mod iostd;
mod list_mahasiswa;
mod tictactoe;
mod tree;

fn main() {
    let mut input_user = String::new();
    let run: bool = true;
    while run {
        println!("1. List Mahasiswa");
        println!("2. Calculator");
        println!("3. Tic Tac Toe");
        println!("4. Blockchain");
        println!("5. Tree");
        println!("6. I/O");
        println!("7. Exit");
        print!("Pilih menu: ");
        io::stdout().flush().unwrap();
        input_user.clear();
        io::stdin().read_line(&mut input_user).unwrap();
        let input_user: Result<u32, _> = input_user.trim().parse();

        match input_user {
            Ok(1) => list_mahasiswa::main(),
            Ok(2) => calculator::main(),
            Ok(3) => tictactoe::main(),
            Ok(4) => blockchain::main(),
            Ok(5) => tree::main(),
            Ok(6) => {
                if let Err(err) = iostd::main() {
                    println!("Error: {:?}", err);
                }
            }
            Ok(7) => break,
            _ => println!("Menu tidak tersedia atau input tidak valid"),
        }
    }
}
