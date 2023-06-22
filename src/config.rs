use crate::error_handler;
use crate::jobfile_parser::parse_jobfile;

pub struct Config {
    pub verbose: bool,
    pub jobs: Vec<Job>,
}

#[derive(Default)]
pub struct Job {
    pub program: String,
    pub stdin: String,
    pub stdout: String,
    pub timeout: u16,
    pub args: Vec<String>,
}

impl Config {
    pub fn build(
        args: impl Iterator<Item = String>,
    ) -> Result<Config, error_handler::JobrunnerError> {
        let input: Vec<String> = args.skip(1).collect();

        if input.is_empty() {
            return Err(error_handler::JobrunnerError {
                error_code: 1,
                ..Default::default()
            });
        }

        let mut config: Config = Config {
            verbose: false,
            jobs: Vec::new(),
        };

        for (index, arg) in input.iter().enumerate() {
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

        Ok(config)
    }
}
