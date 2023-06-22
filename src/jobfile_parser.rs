use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::config;
use crate::error_handler;

pub fn parse_jobfile(file_path: &str) -> Result<Vec<config::Job>, error_handler::JobrunnerError> {
    let Ok(file) = File::open(PathBuf::from(file_path)) else {
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
            error_code: 5,
            text: Some(format!("Error reading lines in {}", file_path)),
            ..Default::default()
        });
    };

    let jobs: Vec<config::Job> = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        if line.starts_with('#') {
            continue;
        } else {
        }
    }
    Ok(jobs)
}

fn check_job_validity(job_string: String) -> Option<config::Job> {
    Some(Default::default())
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn read_file() {
        let Ok(nut) = parse_jobfile("jobfiles/jobfile1") else {
            return assert!(false);
        };
    }
}
