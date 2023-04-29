use scheduler::cfs;
use scheduler::priority_queue;
use scheduler::round_robin;
use scheduler::Scheduler;

use std::env;
use std::fs;

use processor::format_logs;
use processor::Log;
use std::num::NonZeroUsize;

mod deadlock;
mod panic;
mod simple;
mod wait_and_signal;

fn write_logs(folder: &str, name: &str, logs: &str) {
    let scheduler = env::var("SCHEDULER").unwrap();
    let (timeslice, remaining, cpu_slices) = arguments();
    fs::create_dir_all(format!("../outputs/{scheduler}/{folder}")).unwrap();
    fs::write(
        format!(
            "../outputs/{scheduler}/{folder}/{name}___{timeslice}_{remaining}_{cpu_slices}.log"
        ),
        logs,
    )
    .unwrap();
}

fn read_logs(folder: &str, name: &str) -> String {
    let scheduler = env::var("SCHEDULER").unwrap();
    let (timeslice, remaining, cpu_slices) = arguments();
    fs::read_to_string(format!(
        "../outputs/{scheduler}/{folder}/{name}___{timeslice}_{remaining}_{cpu_slices}.log"
    ))
    .unwrap()
}

fn run(folder: &str, name: &str, logs: &[Log]) {
    let output = format_logs(&logs);

    if env::var("WRITE_OUTPUT").is_ok() {
        write_logs(folder, name, &output);
    } else {
        let reference = read_logs(folder, name);

        println!("\nleft = Your Output\nright = Correct Output\n");
        use pretty_assertions::assert_eq;
        assert_eq!(reference, output);
    }
}

fn arguments() -> (usize, usize, usize) {
    let timeslice = env::var("TIMESLICE")
        .unwrap_or("3".to_string())
        .parse::<usize>()
        .unwrap();
    let remaining = env::var("REMAINING")
        .unwrap_or("1".to_string())
        .parse::<usize>()
        .unwrap();
    let cpu_slices = env::var("CPU_SLICES")
        .unwrap_or("10".to_string())
        .parse::<usize>()
        .unwrap();
    (timeslice, remaining, cpu_slices)
}

fn scheduler() -> impl Scheduler {
    let (timeslice, remaining, cpu_slices) = arguments();

    println!("Timeslice {timeslice}\nRemaining {remaining}\nCPU slices: {cpu_slices}");

    match env::var("SCHEDULER").unwrap().as_str() {
        "round-robin" => round_robin(NonZeroUsize::new(timeslice).unwrap(), remaining),
        _ => panic!("unknown scheduler"),
    }
}