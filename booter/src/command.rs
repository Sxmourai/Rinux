use std::process::{Command, ExitStatus};

#[track_caller]
pub fn command(rargs: String, with_command: impl Fn(&mut Command)) -> ExitStatus {
    println!("\t\"{rargs}\"...\r");
    let mut args = rargs.split_ascii_whitespace();
    let program = args.next().unwrap();
    let mut cmd = Command::new(program);
    for arg in args {// TODO Use args
        cmd.arg(arg);
    }
    // cmd.stdout(std::process::Stdio::piped());
    (with_command)(&mut cmd);
    let status = cmd
        .spawn()
        .expect("Failed starting command")
        .wait()
        .expect("Failed executing command");
    if !status.success() {
        let msg = format!("| ❗ ERROR: Executing command \"{rargs}\" |");//⚠️❌🚨
        let marker = "-".repeat(msg.len()-1);
        println!("{}", marker);
        println!("{msg}");
        println!("{}", marker);
    }
    status
}

#[macro_export]
//TODO For now we need to append a , at the end of the calls (i.e. cmd!("abc",)) which isn't great
macro_rules! cmd {
    (dir=$dir:expr, $($arg:tt)*) => {{
        $crate::command(format!($($arg)*), |cmd| {cmd.current_dir($dir);})
    }};
    (panic=$panic_msg:expr,dir=$dir:expr, $($arg:tt)*) => {{
        if !$crate::command(format!($($arg)*), |cmd| {cmd.current_dir($dir);}).success() {
            panic!("Error executing panicable command: {}", $panic_msg);
        }
    }};
    ($($arg:tt)*) => {{
        $crate::command(format!($($arg)*), |_| {})
    }};
}