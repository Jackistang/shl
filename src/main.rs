use std::io::{self, Write};
use std::io::Error;

fn main() -> Result<(), Error> {

    loop {
        print!("shl$ ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let str = line.trim();
        if str == "" {
            continue;
        } else if str == "exit" {
            break ;
        } else {
            println!("Unsupported command");
        }
    }

    Ok(())
}
