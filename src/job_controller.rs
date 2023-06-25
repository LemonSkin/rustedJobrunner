use std::fs;
use std::io;
use std::thread::sleep;
use std::{collections, process, time};

#[derive(Debug)]
struct ProcessData {
    child: process::Child,
    timeout: usize,
    job_number: usize,
    process_finished: bool,
}

// I need to launch only processes that have an output pipe, then launch jobs with both pipes, then launch jobs with no pipes

pub fn run_jobs(config: crate::config::Config) {
    // I want ls | cat > ls.txt
    // cat,@pipe1,ls.txt,
    // ls,-,@pipe1,
    // let mut cat_process = process::Command::new("cat");
    // cat_process.stdout(
    //     fs::OpenOptions::new()
    //         .write(true)
    //         .truncate(true)
    //         .open("ls.out")
    //         .unwrap(),
    // );
    // let mut ls_process = process::Command::new("ls");
    // ls_process.stdout(process::Stdio::piped());

    // let mut meme = process::Command::new("ls")
    //     .stdout(process::Stdio::piped())
    //     .spawn()
    //     .expect("Failed to start meme");

    // let meme_out = meme.stdout.expect("Failed to retrieve");

    // echo_child
    //     .stdin(process::Stdio::from(meme_out))
    //     .spawn()
    //     .expect("Failed to start cat");

    // Command with the Command, the time out and the job number
    let commands: Vec<(process::Command, usize, usize)> = generate_commands(config);

    let mut processes: Vec<ProcessData> = generate_processes(commands);

    let mut jobs_remaining = processes.len();

    let mut elapsed_time: usize = 0;
    while jobs_remaining > 0 {
        for process in &mut processes {
            if !process.process_finished {
                match process.child.try_wait() {
                    Ok(Some(_)) => {
                        eprintln!("Job {} exited with status 0", process.job_number);
                        jobs_remaining -= 1;
                        process.process_finished = true;
                    }
                    Ok(None) => {
                        if elapsed_time > process.timeout && process.timeout > 0 {
                            eprintln!("Job {} terminated", process.job_number);
                            process.child.kill().expect("Failed to kill child");
                            jobs_remaining -= 1;
                            process.process_finished = true;
                        }
                    }
                    Err(_) => {
                        eprintln!("Job {} exited with status 255", process.job_number);
                        jobs_remaining -= 1;
                        process.process_finished = true;
                    }
                }
            }
        }

        if jobs_remaining > 0 {
            sleep(time::Duration::from_secs(1));
            elapsed_time += 1;
        }
    }
}

fn generate_commands(
    mut config: crate::config::Config,
) -> Vec<(std::process::Command, usize, usize)> {
    // Process pipes first
    for (index, job) in config.jobs.iter().enumerate() {
        if job.stdin.starts_with('@') {}
    }

    let mut commands: Vec<(std::process::Command, usize, usize)> = Vec::new();

    for (index, job) in config.jobs.iter_mut().enumerate() {
        if job.runnable {
            let mut command = process::Command::new(&job.program);
            if job.stdin != "-" && !job.stdin.starts_with('@') {
                command.stdin(fs::OpenOptions::new().read(true).open(&job.stdin).unwrap());
            }

            if job.stdout != "-" && !job.stdout.starts_with('@') {
                command.stdout(
                    fs::OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(&job.stdout)
                        .unwrap(),
                );
            } else if job.stdout.starts_with('@') {
                command.stdout(process::Stdio::piped());
                // TODO Create output pipe to be used as input pipe elsewhere
            }

            // TODO Process pipes

            if !job.args.is_empty() {
                command.args(&job.args);
            }

            commands.push((command, job.timeout, index + 1));
        }
    }

    commands
}

fn generate_processes(mut commands: Vec<(process::Command, usize, usize)>) -> Vec<ProcessData> {
    let mut processes = Vec::new();
    for cmd in &mut commands {
        let Ok(child) =  cmd.0.spawn() else {
            eprintln!("Job {} exited with status 255", cmd.2);
            continue;
        };

        let process = ProcessData {
            child,
            timeout: cmd.1,
            job_number: cmd.2,
            process_finished: false,
        };
        processes.push(process);
    }
    processes
}
