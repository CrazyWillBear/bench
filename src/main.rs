use std::env;
use std::process;
use std::process::{Command, Stdio};
use stopwatch::Stopwatch;
use sysinfo::{ProcessorExt, System, SystemExt};


fn main() {
    // Take in commandline args.
    let args: Vec<String> = env::args().collect();
    
    // If no program is inputted to run, simply exit and throw the following
    // error:
    if args.len() < 2 {
        eprintln!("Error: Not enough arguments provided!");
        process::exit(1);
    }
    
    // Program args are the ones that come after the program name, so
    // collect those to be later used when the program is run.
    // NOTICE: THIS CODE MAY BE REPLACED LATER, AS IT IS NOT APPARENTLY
    // NECESSARY!
    let program_args: Vec<String> = args[2..args.len()].to_vec();
    let mut sys = System::new_all();

    println!("\x1b[0;32m-Running '{}':\x1b[0;0m\n-=-=-=-", args[1]);

    // Take a benchmark of the initial RAM and CPU usage, to compare the
    // results with after the program is run. Also start the stopwatch.
    let mut sw = Stopwatch::start_new();
    let init_cpu_usage = ProcessorExt::cpu_usage(sys.global_processor_info());
    let init_memory_used = sys.used_memory();

    // Run the program itself using the args provided by the user. Output
    // standard output, input, and error so that the user can interact
    // with the program while it runs.
    let proc = Command::new(&args[1])
        .args(program_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output()
        .expect("failed to run command");

    // Take a benchmark of the RAM and CPU usage, and stop the stopwatch.
    sys.refresh_all();
    let end_memory_used = sys.used_memory();
    let end_cpu_usage = ProcessorExt::cpu_usage(sys.global_processor_info());
    sw.stop();
    
    // Print final statement containing benchmark results. Info on how CPU
    // usage and RAM usage are determined are commented below.
    print!("-=-=-=-\n\x1b[0;32m");
    println!("-Ran '{}':", &args[1]);
    println!("     -{} ms", sw.elapsed_ms());
    println!("     -memory usage: {}mb", ((end_memory_used - init_memory_used) as f32 / 1000.00)); // the memory used is the difference between the amount of memory used before and after the program was run.
    println!("     -cpu usage: {}%", (end_cpu_usage - init_cpu_usage) as i32); // the CPU usage is the difference between the CPU usage before ad after the program was run.
    println!("     -{}\x1b[0;0m", &proc.status);
}
