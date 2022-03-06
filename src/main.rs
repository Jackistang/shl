use std::io::{self, Write};


fn main() -> io::Result<()> {
    println!("Hello, shl!");

    loop {
        print!("shl$ ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let str = line.trim();
        if str == "" {
            continue;
        } else if str == "exit" {
            return Ok(());
        } else {
            println!("Unsupported command");
        }
    }
}
