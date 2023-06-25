use std::process;

#[derive(Default)]
pub struct JobrunnerError {
    pub error_code: i32,
    pub text: Option<String>,
    pub line_num: Option<usize>,
}

pub fn handle(error: JobrunnerError) {
    match error.error_code {
        1 => eprintln!("Usage: jobrunner [-v] jobfile [jobfile ...]"),
        2 => eprintln!(
            "jobrunner: file \"{}\" can not be opened",
            error.text.unwrap()
        ),
        3 => eprintln!(
            "jobrunner: invalid job specification on line {} of \"{}\"",
            error.line_num.unwrap(),
            error.text.unwrap()
        ),
        4 => eprintln!("jobrunner: no runnable jobs"),
        _ => {
            if let Some(err_text) = error.text {
                eprintln!("jobrunner: UNHANDLED ERROR: {}", err_text);
            } else {
                eprintln!("jobrunner: UNHANDLED ERROR");
            }
        }
    }

    process::exit(error.error_code);
}
