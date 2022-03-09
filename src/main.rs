use std::process::{Command};

use rustyline::{Editor, Result};
use clap::{Parser};

#[derive(Parser)]
struct Args {
    /// Test flag
    #[clap(long)]
    test: bool
}

fn main() -> Result<()> {
    let args = Args::parse();

    let prompt; 
    match args.test {
        true  => prompt = String::from(""),
        false => prompt = String::from("shl$ "),
    };

    let mut rl = Editor::<()>::new();
    loop {
        let line = rl.readline(&prompt)?;
        let args: Vec<&str> = line.trim().split(' ').collect();

        let cmd = String::from(args[0]);
        if cmd == "" {
            continue;
        } else if cmd == "exit" {
            break ;
        } else if cmd == "cd" {
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
