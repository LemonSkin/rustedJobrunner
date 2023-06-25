use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::error_handler;

pub fn parse_jobfile(file_path: &str) -> Result<Vec<crate::Job>, error_handler::JobrunnerError> {
    let Ok(file) = fs::File::open(PathBuf::from(file_path)) else {
        return Err(error_handler::JobrunnerError {
            error_code: 2,
            text: Some(file_path.to_string()),
            ..Default::default()
        });
    };

    let Ok(lines) = BufReader::new(&file)
    .lines()
    .collect::<Result<Vec<String>, _>>() else {
        return Err(error_handler::JobrunnerError {
            error_code: 99,
            text: Some(format!("Error reading lines in {}", file_path)),
            ..Default::default()
        });
    };

    let mut jobs: Vec<crate::Job> = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        } else {
            let Some(job) = check_job_validity(line) else {
                return Err(error_handler::JobrunnerError {
                    error_code: 3,
                    text: Some(file_path.to_string()),
                    line_num: Some(line_num+1)
                })
            };
            jobs.push(job);
        }
    }

    // println!("{:?}", jobs);
    Ok(jobs)
}

fn check_job_validity(job_string: &str) -> Option<crate::Job> {
    let fields: Vec<&str> = job_string.split(',').collect();

    if fields.len() < 3 {
        return None;
    }

    let mut job: crate::Job = Default::default();

    for (index, field) in fields.iter().enumerate() {
        match index {
            0..=2 => {
                if field.is_empty() {
                    return None;
                } else {
                    match index {
                        0 => job.program = field.to_string(),
                        1 => job.stdin = field.to_string(),
                        2 => job.stdout = field.to_string(),
                        _ => (),
                    }
                }
            }
            3 => {
                // Timeout is initialised to 0 so we can ignore if the field exists but is empty
                if field.is_empty() {
                    continue;
                } else {
                    job.timeout = match field.parse::<usize>() {
                        Ok(timeout) => timeout,
                        _ => return None,
                    }
                }
            }
            _ => job.args.push(field.to_string()),
        };
    }

    Some(job)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn read_file() {
        let Ok(jobs) = parse_jobfile("jobfiles/jobfile1") else {
            return assert!(false);
        };
    }

    #[test]
    fn job_syntax_valid() {
        let Some(job) = check_job_validity("ls,-,-,0") else {
            return assert!(false);
        };
    }

    #[test]
    fn job_syntax_invalid() {
        let Some(job) = check_job_validity("ls,-") else {
            return assert!(true);
        };
    }
}
