use std::fs::{self, File};
use std::io::{self, Read, Write};

pub fn main() -> io::Result<()> {
    let mut file_path = String::new();
    print!("Enter file path: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut file_path)?;
    let file_path = file_path.trim();

    if fs::metadata(file_path).is_ok() {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        println!("File content: {}", contents);
    } else {
        let mut file = File::create(file_path)?;
        let mut content = String::new();
        println!("Enter file content (press `Ctrl+D` to finish):");
        io::stdin().read_to_string(&mut content)?;
        file.write_all(content.as_bytes())?;
        println!("File created with content {}", content);
    }
    Ok(())
}
