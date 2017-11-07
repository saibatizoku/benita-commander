//! Reusable command-line items.
use clap::{App, AppSettings, Arg, SubCommand};

/// responder subcommand that reads the `URL`, `I2C_PATH`, and `I2C_ADDRESS`
/// needed for the service.
pub fn sensor_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("sensor")
        .about("send commands directly over I2C to sensor")
        .args(&[
            Arg::with_name("I2C_PATH").required(true),
            Arg::with_name("I2C_ADDRESS").required(true),
        ])
}

/// responder subcommand that reads the `URL`, `I2C_PATH`, and `I2C_ADDRESS`
/// needed for the service.
pub fn responder_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("responder")
        .about("REP server")
        .args(&[
            Arg::with_name("URL").required(true),
            Arg::with_name("I2C_PATH").required(true),
            Arg::with_name("I2C_ADDRESS").required(true),
        ])
}

/// Parser for the main program
pub struct BenitaCommanderApp;

impl BenitaCommanderApp {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        App::new("benita-commander")
            .settings(&[AppSettings::SubcommandRequired])
            .subcommands(vec![
                SubCommand::with_name("conductivity")
                    .about("Commands for conductivity")
                    .settings(&[AppSettings::SubcommandRequired])
                    .subcommands(vec![responder_subcommand()]),
                SubCommand::with_name("ph")
                    .about("Commands for pH")
                    .settings(&[AppSettings::SubcommandRequired])
                    .subcommands(vec![responder_subcommand()]),
                SubCommand::with_name("temperature")
                    .about("Commands for temperature")
                    .settings(&[AppSettings::SubcommandRequired])
                    .subcommands(vec![responder_subcommand()]),
            ])
    }
}
