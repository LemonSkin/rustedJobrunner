use crate::error_handler;

mod jobfile_parser;
use jobfile_parser::parse_jobfile;

#[derive(Debug)]
pub struct Config {
    pub verbose: bool,
    pub jobs: Vec<crate::Job>,
    pub runnable_jobs: usize,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, error_handler::JobrunnerError> {
        if args.is_empty() {
            return Err(error_handler::JobrunnerError {
                error_code: 1,
                ..Default::default()
            });
        }

        let mut config: Config = Config {
            verbose: false,
            jobs: Vec::new(),
            runnable_jobs: 0,
        };

        for (index, arg) in args.iter().enumerate() {
            if arg == "-v" {
                match index {
                    0 => config.verbose = true,
                    _ => {
                        return Err(error_handler::JobrunnerError {
                            error_code: 1,
                            ..Default::default()
                        })
                    }
                }
            } else {
                config.jobs.append(&mut parse_jobfile(arg)?);
            }
        }

        if config.jobs.is_empty() {
            return Err(error_handler::JobrunnerError {
                error_code: 4,
                ..Default::default()
            });
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_config() {
        let Ok(_config) = Config::build(vec!("jobfiles/jobfile1".to_string())) else {
            return assert!(false);
        };
        let Ok(_config) = Config::build(vec!("jobfiles/invalid_jobfile1".to_string())) else {
            return assert!(true);
        };
    }
}
