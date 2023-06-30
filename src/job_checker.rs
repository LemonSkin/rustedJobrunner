use std::collections;
use std::fs::OpenOptions;

use crate::error_handler;

pub fn check_jobs(
    mut config: &mut crate::config::Config,
) -> Result<(), error_handler::JobrunnerError> {
    check_file_io(config);
    check_pipes(config);

    // Calculate number of runnable jobs
    for job in config.jobs.iter() {
        if job.runnable {
            config.runnable_jobs += 1;
        }
    }

    if config.runnable_jobs == 0 {
        return Err(error_handler::JobrunnerError {
            error_code: 4,
            ..Default::default()
        });
    }

    if config.verbose {
        print_runnable_jobs(&config);
    }

    Ok(())
}

fn check_file_io(config: &mut crate::config::Config) {
    for (index, job) in config.jobs.iter_mut().enumerate() {
        // Attempt to open for stdin
        if !job.stdin.starts_with('@') && job.stdin != "-" {
            match OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(&job.stdin)
            {
                Ok(_) => (),
                Err(_) => {
                    eprintln!("Unable to open \"{}\" for reading", job.stdin);
                    job.runnable = false;
                    continue;
                }
            };
        }

        // Attempt to open for stdout - will also create the file for later
        if !job.stdout.starts_with('@') && job.stdout != "-" {
            match OpenOptions::new()
                .read(false)
                .write(true)
                .create(true)
                .truncate(true)
                .open(&job.stdout)
            {
                Ok(_) => (),
                Err(_) => {
                    eprintln!("Unable to open \"{}\" for writing", job.stdout);
                    job.runnable = false;
                    continue;
                }
            };
        }
    }
}

fn check_pipes(config: &mut crate::config::Config) {
    let mut in_pipes_hash: collections::HashMap<String, usize> = collections::HashMap::new();
    let mut out_pipes_hash: collections::HashMap<String, usize> = collections::HashMap::new();

    let mut jobs_to_disable: Vec<usize> = Vec::new();
    // Process duplicated input or output pipes
    for (index, job) in config.jobs.iter_mut().enumerate() {
        if !job.runnable {
            continue;
        }
        if job.stdin.starts_with('@') {
            if !in_pipes_hash.contains_key(&job.stdin) {
                in_pipes_hash.insert(job.stdin.to_string(), index);
            } else {
                // Since key exists, it is a duplicated pipe and needs to be disabled
                job.runnable = false;
                eprintln!("Invalid pipe usage \"{}\"", job.stdin);
                // Need to also disable other jobs that use this pipe - processed later
                jobs_to_disable.push(*in_pipes_hash.get(&job.stdin).unwrap());
            }
        }
        if job.stdout.starts_with('@') {
            if !out_pipes_hash.contains_key(&job.stdout) {
                out_pipes_hash.insert(job.stdout.to_string(), index);
            } else {
                // Since key exists, it is a duplicated pipe and needs to be disabled
                job.runnable = false;
                eprintln!("Invalid pipe usage \"{}\"", job.stdout);
                // Need to also disable other jobs that use this pipe - processed later
                jobs_to_disable.push(*out_pipes_hash.get(&job.stdout).unwrap());
            }
        }
    }
    // Disable jobs with duplicated pipes
    for index in jobs_to_disable {
        config.jobs[index].runnable = false;
    }

    let mut input_pipes: Vec<(String, usize)> = Vec::new();
    let mut output_pipes: Vec<(String, usize)> = Vec::new();
    // Check input pipes have a matching output pipe and disable if not
    for (index, job) in config.jobs.iter().enumerate() {
        if !job.runnable {
            continue;
        }

        if job.stdin.starts_with('@') {
            input_pipes.push((job.stdin.to_string(), index));
        }

        if job.stdout.starts_with('@') {
            output_pipes.push((job.stdout.to_string(), index));
        }
    }

    for i_pipe in &input_pipes {
        let mut exit_pipe_found: bool = false;
        for o_pipe in &output_pipes {
            if o_pipe.0 == i_pipe.0 && config.jobs[o_pipe.1].runnable {
                exit_pipe_found = true;
                break;
            }
        }
        if !exit_pipe_found {
            config.jobs[i_pipe.1].runnable = false;
        }
    }

    for o_pipe in &output_pipes {
        let mut entrance_pipe_found: bool = false;

        for i_pipe in &input_pipes {
            if i_pipe.0 == o_pipe.0 && config.jobs[i_pipe.1].runnable {
                entrance_pipe_found = true;
                break;
            }
        }
        if !entrance_pipe_found {
            config.jobs[o_pipe.1].runnable = false;
        }
    }
}

fn print_runnable_jobs(config: &crate::config::Config) {
    for (index, job) in config.jobs.iter().enumerate() {
        if job.runnable {
            let mut job_string = format!(
                "{}:{}:{}:{}:{}",
                index + 1,
                job.program,
                job.stdin,
                job.stdout,
                job.timeout
            );

            if !job.args.is_empty() {
                for arg in &job.args {
                    job_string.push_str(&format!(":{}", arg));
                }
            }
            println!("{job_string}");
        }
    }
}
