use std::env;

use jobrunner::config;
use jobrunner::error_handler::handle;
use jobrunner::job_checker::check_jobs;
use jobrunner::job_controller::run_jobs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut config = match config::Config::build(args) {
        Ok(config) => config,
        Err(e) => return handle(e),
    };

    match check_jobs(&mut config) {
        Ok(_) => (),
        Err(e) => handle(e),
    };

    run_jobs(config);
}
