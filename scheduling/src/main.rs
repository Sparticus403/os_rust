use std::error::Error;
use std::{env, fs};

#[derive(Debug)]
struct Process {
    sub_time: i32,
    run_time: i32,
    start_time: i32,
    remaining_run_time: i32,
}

impl Process {
    fn from(tup: (i32, i32)) -> Self {
        Process {
            sub_time: tup.0,
            run_time: tup.1,
            start_time: -1,
            remaining_run_time: -1,
        }
    }
    fn reset(&mut self) {
        self.start_time = -1;
        self.remaining_run_time = -1;
    }
}
fn create_process_list(procs: &[(i32, i32)]) -> Vec<Process> {
    let mut processes: Vec<Process> = Vec::new();

    for i in 0..procs.len() {
        let proc: Process = Process::from(procs[i]);
        processes.push(proc);
    }

    processes
}

fn fcfs(procs: &mut [Process]) {
    println!("First Come, First Served");
    let mut clock = 0;
    let mut total_turn_around = 0;
    let mut total_wait = 0;
    let mut total_response = 0;

    for i in 0..procs.len() {
        if clock < procs[i].sub_time {
            clock = procs[i].sub_time;
        }
        procs[i].start_time = clock;
        clock += procs[i].run_time;
        procs[i].remaining_run_time = 0;
        total_turn_around += clock - procs[i].sub_time;
        total_wait += total_turn_around - procs[i].start_time;
        total_response += total_turn_around - procs[i].start_time;
    }
    println!(
        "Avg TA: {}, Avg Wait: {}, Avg Resp: {}",
        total_turn_around / procs.len() as i32,
        total_wait / procs.len() as i32,
        total_response / procs.len() as i32
    );
}

fn round_robin(procs: &mut [Process]) {

}

fn sjf(procs: &mut [Process]) {

}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    //println!("file: {}", file);
    let contents = fs::read_to_string(file).expect("Couldn't read file");
    //println!("contents: {contents}\n");

    let arr: Vec<_> = contents.split_whitespace().collect();

    let processes = create_process_vec(&arr);
    //println!("{:?}", processes);
    let mut processes = create_process_list(&processes);
    fcfs(&mut processes);
    for i in 0..processes.len() {
        processes[i].reset();
    }
    //println!("{:#?}", processes);
    Ok(())
}

fn create_process_vec(args: &[&str]) -> Vec<(i32, i32)> {
    let mut processes: Vec<(i32, i32)> = Vec::new();
    let mut tup: (i32, i32) = (-1, -1);

    for i in 0..args.len() {
        if i % 2 == 0 {
            tup.0 = args[i].trim().parse().expect("could not parse string");
        } else {
            tup.1 = args[i].trim().parse().expect("could not parse string");
            processes.push(tup);
        }
    }
    processes
}
