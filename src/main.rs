use std::io::{self, Write};
mod calculator;
mod list_mahasiswa;

fn main() {
    let mut input_user = String::new();
    let run: bool = true;
    while run {
        println!("1. List Mahasiswa");
        println!("2. Calculator");
        println!("3. Exit");
        print!("Pilih menu: ");
        io::stdout().flush().unwrap();
        input_user.clear();
        io::stdin().read_line(&mut input_user).unwrap();
        let input_user: Result<u32, _> = input_user.trim().parse();

        match input_user {
            Ok(1) => list_mahasiswa::main(),
            Ok(2) => calculator::main(),
            Ok(3) => break,
            _ => println!("Menu tidak tersedia atau input tidak valid"),
        }
    }
}
