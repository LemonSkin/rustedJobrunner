use std::fs;
use std::thread::sleep;
use std::{collections, process, time};

#[derive(Debug)]
struct ProcessData {
    child: process::Child,
    timeout: usize,
    job_number: usize,
    process_finished: bool,
}

// Launch jobs with no pipes, then output pipe, then input pipe
pub fn run_jobs(config: crate::config::Config) {
    let mut processes: Vec<ProcessData> = generate_processes(config);

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

fn generate_processes(mut config: crate::config::Config) -> Vec<ProcessData> {
    // Count number of output pipes for later
    let mut outpipes_remaining = 0;
    for job in &config.jobs {
        if job.runnable && job.stdout.starts_with('@') {
            outpipes_remaining += 1;
        }
    }
    // Generate a pipe mapping to connect input and output pipes
    let mut pipe_map: collections::HashMap<String, usize> = collections::HashMap::new();

    let mut processes: Vec<ProcessData> = Vec::new();

    while config.runnable_jobs > 0 {
        for (index, job) in config.jobs.iter_mut().enumerate() {
            if job.runnable {
                // 1. If job contains an input pipe, first check that the corresponding
                //    output pipe has been created and skip until later if not
                if job.stdin.starts_with('@') {
                    // Ensure there are active output pipes
                    match pipe_map.get(&job.stdin) {
                        Some(_) => (),
                        None => {
                            if outpipes_remaining == 0 {
                                eprintln!("Job {} failed: Broken pipe dependency", index + 1);
                                job.runnable = false;
                                config.runnable_jobs -= 1;
                            }
                            continue;
                        }
                    }
                }

                let mut command = process::Command::new(&job.program);
                // Set stdin to default, file IO or pipe
                if job.stdin != "-" && !job.stdin.starts_with('@') {
                    command.stdin(fs::OpenOptions::new().read(true).open(&job.stdin).unwrap());
                } else if job.stdin.starts_with('@') {
                    //Find the appropriate out pipe
                    let location = pipe_map.get(&job.stdin).unwrap();
                    let stdout = processes[*location].child.stdout.take().unwrap();
                    command.stdin(process::Stdio::from(stdout));
                }

                // Set stdout to default, file IO or pipe
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
                    pipe_map.insert(job.stdout.clone(), pipe_map.len());
                    outpipes_remaining -= 1;
                }

                if !job.args.is_empty() {
                    command.args(&job.args);
                }

                let Ok(child) = command.spawn() else {
                    eprintln!("Job {} exited with status 255", index + 1);
                    job.runnable = false;
                    config.runnable_jobs -= 1;
                    continue;
                };

                processes.push(ProcessData {
                    child,
                    timeout: job.timeout,
                    job_number: index + 1,
                    process_finished: false,
                });

                job.runnable = false;
                config.runnable_jobs -= 1;
            }
        }
    }

    processes
}
