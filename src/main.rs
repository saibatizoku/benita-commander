//! Control interface for `benita` services.
#![recursion_limit = "1024"]

extern crate benita;
extern crate chrono;
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate fern;
#[macro_use]
extern crate log;
extern crate rustyline;

pub mod cli;
pub mod errors {
    //! Errors and their descriptions
    use super::logging;
    use super::readline;
    use benita;

    error_chain! {
        errors {
            InvalidArgs {
                description ("invalid command-line arguments")
            }
        }
        links {
            Benita(benita::errors::Error, benita::errors::ErrorKind);
            Logs(logging::Error, logging::ErrorKind);
            Readline(readline::Error, readline::ErrorKind);
        }
    }
}
pub mod logging;
pub mod readline;

use self::errors::*;
use cli::{responder_subcommand, BenitaCommanderApp};
use logging::start_log;

use clap::{App, AppSettings, ArgMatches, SubCommand};


/// Execute the program from the given command-line arguments
fn evaluate(args: &ArgMatches) -> Result<()> {
    // This runs the whole program. Everything has brought us to this point.
    // Use it wisely.
    debug!("{:?}", args);
    println!("Hello, world!");
    Ok(())
}

/// parse command line arguments, return the valid matches.
fn parse_command_line<'c>() -> Result<ArgMatches<'c>> {
    // Setup the app that will parse the command-line.
    let parser_app = BenitaCommanderApp::new();
    let matches = parser_app.get_matches();
    Ok(matches)
}

/// Main program. Starts logger, then evaluates args from stdin.
fn run_code() -> Result<()> {
    {
        // Parse the user input from the command-line.
        let matched_args = parse_command_line()?;

        // Initialize logging.
        let _log = start_log()?;

        info!("benita-command starting");
        let _run = evaluate(&matched_args)?;
    }

    warn!("About to stop calibrated-service");
    info!("benita-command stopping");
    Ok(())
}

// fn main() wrapped to handle error chains
quick_main!(run_code);
