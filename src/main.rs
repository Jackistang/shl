use std::io::{self, Write};
use std::io::Error;
use std::process::{Command};

pub fn set_prompt(prompt: &str) {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
}

fn main() -> Result<(), Error> {
    let stdout = io::stdout();
    loop {
        set_prompt("");

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let args: Vec<&str> = line.trim().split(' ').collect();
        let cmd = &args[0];

        if cmd == &"" {
            continue;
        } else if cmd == &"exit" {
            break ;
        } else if cmd == &"cd" {
            match std::env::set_current_dir(args[1]) {
                Err(err)    =>   {
                    println!("cd: {}: {}", args[1], err);
                },
                Ok(_)        =>  { },
            };
        } else {
            // 创建一个子进程执行该程序
            let output = Command::new(args[0])
                                .args(&args[1..])
                                .output();
            // 判断该程序是否正确执行
            match output {
                Ok(res)     =>  {
                    print!("{}", String::from_utf8(res.stdout).unwrap());
                    print!("{}", String::from_utf8(res.stderr).unwrap());
                },
                _           =>  println!("{}: command not found", args[0]),
            };
        }
    }

    Ok(())
}
