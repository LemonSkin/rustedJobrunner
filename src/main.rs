use std::env;

mod config;
mod error_handler;
mod jobfile_parser;

use error_handler::handle;

fn main() {
    let config = match config::Config::build(env::args()) {
        Ok(config) => config,
        Err(e) => return handle(e),
    };

    // handle(error_handler::JobrunnerError {
    //     error_code: 1,
    //     file_name: None,
    //     line_num: None,
    // });
}
