use std::env;
use std::process::{Command, Stdio};
use stopwatch::Stopwatch;
use sysinfo::{ProcessorExt, System, SystemExt};


fn main() {
    let args: Vec<String> = env::args().collect();
    let program_args: Vec<String> = args[2..args.len()].to_vec();
    let mut sys = System::new_all();

    println!("\x1b[0;32m-Running '{}':\x1b[0;0m\n-=-=-=-", args[1]);

    let mut sw = Stopwatch::start_new();
    let init_cpu_usage = ProcessorExt::cpu_usage(sys.global_processor_info());
    let init_memory_used = sys.used_memory();

    let proc = Command::new(&args[1])
        .args(program_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output()
        .expect("failed to run command");

    sys.refresh_all();
    let end_memory_used = sys.used_memory();
    let end_cpu_usage = ProcessorExt::cpu_usage(sys.global_processor_info());
    sw.stop();

    print!("-=-=-=-\n\x1b[0;32m");
    println!("-Ran '{}':", &args[1]);
    println!("     -{} ms", sw.elapsed_ms());
    println!("     -memory usage: {}mb", ((end_memory_used - init_memory_used) as f32 / 1000.00));
    println!("     -cpu usage: {}%", (end_cpu_usage - init_cpu_usage) as i32);
    println!("     -{}\x1b[0;0m", &proc.status);
}