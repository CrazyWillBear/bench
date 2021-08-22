use stopwatch;

use std::{env, io};
use std::process::Command;
use stopwatch::Stopwatch;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_args: Vec<String> = args[2..args.len()].to_vec();

    let mut sw = Stopwatch::start_new();

    println!("\x1b[0;32m-Running '{}':\x1b[0;0m\n-=-=-=-", args[1]);
    let output = Command::new(&args[1])
        .args(program_args)
        .output()
        .expect("failed to execute process");
    io::stdout().write(&output.stdout).unwrap();
    io::stdout().write(&output.stderr).unwrap();

    sw.stop();

    println!("-=-=-=-\n\x1b[0;32m-Ran '{}'\n     -{} ms\n     -{}\x1b[0;0m", &args[1], sw.elapsed_ms(), &output.status);
}