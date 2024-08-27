use std::io::{self, Write};

struct Mahasiswa {
    nim: u32,
    nama: String,
    nilai: f64,
}

struct DaftarMahasiswa {
    mahasiswa: Vec<Mahasiswa>,
}

pub fn main() {
    let mut banyak_mahasiswa = String::new();
    let mut input_user = String::new();
    let mut daftar_mahasiswa = DaftarMahasiswa {
        mahasiswa: Vec::new(),
    };

    loop {
        println!("1. Input mahasiswa");
        println!("2. Print mahasiswa");
        println!("3. Exit");
        print!("Pilih menu: ");
        io::stdout().flush().unwrap();
        input_user.clear();
        io::stdin().read_line(&mut input_user).unwrap();
        let input_user: Result<u32, _> = input_user.trim().parse();

        match input_user {
            Ok(1) => input_mahasiswa(&mut daftar_mahasiswa, &mut banyak_mahasiswa),
            Ok(2) => print_mahasiswa(&daftar_mahasiswa),
            Ok(3) => break,
            _ => println!("Menu tidak tersedia atau input tidak valid"),
        }
    }
}

fn print_mahasiswa(daftar_mahasiswa: &DaftarMahasiswa) {
    println!("======================");
    println!("|| Daftar Mahasiswa ||");
    println!("======================");
    if daftar_mahasiswa.mahasiswa.is_empty() {
        println!("Tidak ada mahasiswa yang dimasukkan");
        return;
    }
    for (i, mahasiswa) in daftar_mahasiswa.mahasiswa.iter().enumerate() {
        println!("Mahasiswa ke-{}", i + 1);
        println!("NIM: {}", mahasiswa.nim);
        println!("Nama: {}", mahasiswa.nama);
        println!("Nilai: {}", mahasiswa.nilai);
    }
}

fn input_mahasiswa(daftar_mahasiswa: &mut DaftarMahasiswa, banyak_mahasiswa: &mut String) {
    print!("Masukkan banyak mahasiswa: ");
    io::stdout().flush().unwrap();
    banyak_mahasiswa.clear();
    io::stdin().read_line(banyak_mahasiswa).unwrap();
    let banyak_mahasiswa: u32 = match banyak_mahasiswa.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input tidak valid");
            return;
        }
    };

    for i in 0..banyak_mahasiswa {
        let mut nim = String::new();
        let mut nama = String::new();
        let mut nilai = String::new();

        print!("Masukkan NIM mahasiswa ke-{}: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut nim).unwrap();
        let nim: u32 = match nim.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input NIM tidak valid");
                continue;
            }
        };

        print!("Masukkan nama mahasiswa ke-{}: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut nama).unwrap();
        let nama = nama.trim().to_string();

        print!("Masukkan nilai mahasiswa ke-{}: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut nilai).unwrap();
        let nilai: f64 = match nilai.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input nilai tidak valid");
                continue;
            }
        };

        let mahasiswa = Mahasiswa { nim, nama, nilai };
        daftar_mahasiswa.mahasiswa.push(mahasiswa);
    }
}
