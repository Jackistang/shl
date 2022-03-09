use std::env;

mod common;

// TODO 给 shl 添加一个 --test 选项，用于将提示符 prompt 取消，方便测试。

#[test]
fn cd() {
    // TODO 获取环境变量 Home，通过 cd 命令切换到 Home 目录，并用 pwd 检测。
    let home = env::var("HOME").unwrap();

    common::exec_command(
        // stdin
        &format!("cd {}\n pwd \n exit", home), 
        // stdout
        &format!("{}\n", home),
    );
}


#[test]
fn extern_command() {
    let path = env::current_dir().unwrap();

    common::exec_command(
        "pwd \n exit",
        &format!("{}\n", path.to_str().unwrap()),
    );
}