pub mod config;
pub mod error_handler;
pub mod job_checker;
pub mod job_controller;

#[derive(Debug)]
pub struct Job {
    pub program: String,
    pub stdin: String,
    pub stdout: String,
    pub timeout: usize,
    pub args: Vec<String>,
    pub runnable: bool,
}

impl Default for Job {
    fn default() -> Job {
        Job {
            program: Default::default(),
            stdin: Default::default(),
            stdout: Default::default(),
            timeout: 0,
            args: Default::default(),
            runnable: true,
        }
    }
}
