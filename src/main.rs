//! Control interface for `benita` services.
#![recursion_limit = "1024"]

extern crate benita;
extern crate chrono;
extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate fern;

/// Errors and their descriptions
pub mod errors {
    use super::logging;
    use benita;

    error_chain! {
        errors {
            InvalidArgs {
                description ("invalid command-line arguments")
            }
        }
        links {
            Logs(logging::Error, logging::ErrorKind);
            Benita(benita::errors::Error, benita::errors::ErrorKind);
        }
    }
}
pub mod logging;

use self::errors::*;
use logging::start_log;

use clap::{App, AppSettings, ArgMatches, SubCommand};


/// Execute the program from the given command-line arguments
fn evaluate(args: &ArgMatches) ->Result<()> {
    debug!("{:?}", args);
    println!("Hello, world!");
    Ok(())
}

/// parse command line arguments, return the valid matches.
fn parse_command_line<'a>() -> Result<ArgMatches<'a>> {
    let cli_parser = App::new("benita-commander")
            .settings(&[
                // AppSettings::DisableHelpSubcommand,
                AppSettings::SubcommandRequired,
            ])
            .subcommands(vec![
                         SubCommand::with_name("conductivity"),
                         SubCommand::with_name("ph"),
                         SubCommand::with_name("temperature"),
                         ]);
    let matches = cli_parser
        .get_matches_safe()
        .chain_err(|| ErrorKind::InvalidArgs)?;
    Ok(matches)
}

/// Main program. Starts logger, then evaluates args from stdin.
fn run_code() -> Result<()> {
    // Initialize logging.
    let _log = start_log()?;

    info!("benita-command starting");

    // This runs the whole program. Everything has brought us to this point.
    // Use it wisely.
    {
        // Parse the user input from the command-line.
        let matched_args = parse_command_line()?;
        let _run = evaluate(&matched_args)?;
    }

    warn!("About to stop calibrated-service");
    info!("benita-command stopping");
    Ok(())
}

// fn main() wrapped to handle error chains
quick_main!(run_code);
