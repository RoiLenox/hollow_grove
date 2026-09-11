use std::env;
use std::process::ExitCode;

use hollow_grove::client::{ClientOptions, run_client};

fn main() -> ExitCode {
    match parse_options(env::args().skip(1)) {
        Ok(Some(options)) => match run_client(options) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("hollow-grove-client: {error}");
                ExitCode::FAILURE
            }
        },
        Ok(None) => {
            println!("{}", usage());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("hollow-grove-client: {error}\n\n{}", usage());
            ExitCode::FAILURE
        }
    }
}

fn parse_options(args: impl IntoIterator<Item = String>) -> Result<Option<ClientOptions>, String> {
    let mut options = ClientOptions::default();
    for argument in args {
        match argument.as_str() {
            "--smoke-test" => options.exit_after_first_frame = true,
            "--help" | "-h" => return Ok(None),
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    Ok(Some(options))
}

fn usage() -> &'static str {
    "Usage: hollow-grove-client [--smoke-test]\n\
     \n\
     Controls:\n\
       Arrow keys / WASD  move and face the Hueman\n\
       Escape             close the client\n\
     \n\
     --smoke-test exits after the first successfully presented frame."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_flag_and_help_parse_without_starting_a_window() {
        assert_eq!(
            parse_options(["--smoke-test".into()]),
            Ok(Some(ClientOptions {
                exit_after_first_frame: true
            }))
        );
        assert_eq!(parse_options(["--help".into()]), Ok(None));
        assert!(parse_options(["--unknown".into()]).is_err());
    }
}
