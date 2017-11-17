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
pub fn responder_subcommand<'a, 'b>(
    url_env: &'a str,
    path_env: &'a str,
    addr_env: &'a str,
) -> App<'a, 'b> {
    SubCommand::with_name("rep").about("REP server").args(&[
        Arg::with_name("URL").env(url_env).required(true),
        Arg::with_name("I2C_PATH").env(path_env).required(true),
        Arg::with_name("I2C_ADDRESS").env(addr_env).required(true),
    ])
}

/// requester subcommand that reads the `URL`
/// needed for the service.
pub fn requester_subcommand<'a, 'b>(env_var: &'a str) -> App<'a, 'b> {
    SubCommand::with_name("req")
        .about("REQ client")
        .arg(Arg::with_name("URL").env(env_var).required(true))
        .arg(
            Arg::with_name("cmds")
                .short("c")
                .multiple(true)
                .takes_value(true)
                .required(false),
        )
}

/// Parser for the main program
pub struct BenitaCommanderApp;

impl BenitaCommanderApp {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        App::new("benita-commander")
            .settings(&[AppSettings::ArgsNegateSubcommands])
            .subcommands(vec![
                SubCommand::with_name("conductivity")
                    .about("Commands for conductivity")
                    .subcommands(
                        vec![
                            requester_subcommand("CONDUCTIVITY_REQ_URL"),
                            responder_subcommand(
                                "CONDUCTIVITY_REP_URL",
                                "CONDUCTIVITY_REP_PATH",
                                "CONDUCTIVITY_REP_ADDRESS",
                                ),
                            ]
                        ),
                SubCommand::with_name("ph")
                    .about("Commands for pH")
                    .subcommands(
                        vec![
                            requester_subcommand("PH_REQ_URL"),
                            responder_subcommand(
                                "PH_REP_URL",
                                "PH_REP_PATH",
                                "PH_REP_ADDRESS",
                                ),
                        ]),
                SubCommand::with_name("temperature")
                    .about("Commands for temperature")
                    .subcommands(
                        vec![
                            requester_subcommand("TEMPERATURE_REQ_URL"),
                            responder_subcommand(
                                "TEMPERATURE_REP_URL",
                                "TEMPERATURE_REP_PATH",
                                "TEMPERATURE_REP_ADDRESS",
                                ),
                            ]),
            ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_valid {
        ( $app_name:ident , $cli:expr ) => {
            let matches = $app_name.get_matches_from_safe_borrow( $cli );
            println!("{:?}", &matches);
            assert!(matches.is_ok());
        };
    }

    macro_rules! test_invalid {
        ( $app_name:ident , $cli:expr ) => {
            let matches = $app_name.get_matches_from_safe_borrow( $cli );
            println!("{:?}", &matches);
            assert!(matches.is_err());
        };
    }

    #[test]
    fn matching_benita_commander_app_valid_subcommands() {
        let mut app = BenitaCommanderApp::new();
        test_valid!(app, &["benita-commander"]);
        test_valid!(app, &["benita-commander"]);
    }

    #[test]
    fn matching_benita_commander_app_invalid_arguments_yields_err() {
        let mut app = BenitaCommanderApp::new();
        test_invalid!(app, &["benita-commander", "wrongcommand"]);
    }

    #[test]
    fn matching_conductivity_valid_responder_subcommands() {
        let mut app = BenitaCommanderApp::new();
        test_valid!(app, &["benita-commander", "conductivity"]);
        test_valid!(
            app,
            &[
                "benita-commander",
                "conductivity",
                "rep",
                "url",
                "path",
                "i2c"
            ]
        );
    }

    #[test]
    fn matching_conductivity_invalid_responder_subcommands_yields_err() {
        let mut app = BenitaCommanderApp::new();
        test_invalid!(app, &["benita-commander", "conductivity", "rep"]);
        test_invalid!(
            app,
            &["benita-commander", "conductivity", "rep", "url", "path"]
        );
        test_invalid!(
            app,
            &[
                "benita-commander",
                "conductivity",
                "rep",
                "url",
                "path",
                "i2c",
                "extra"
            ]
        );
    }

    #[test]
    fn matching_ph_valid_responder_subcommands() {
        let mut app = BenitaCommanderApp::new();
        test_valid!(app, &["benita-commander", "ph"]);
        test_valid!(
            app,
            &["benita-commander", "ph", "rep", "url", "path", "i2c"]
        );
    }

    #[test]
    fn matching_ph_invalid_responder_subcommands_yields_err() {
        let mut app = BenitaCommanderApp::new();
        test_invalid!(app, &["benita-commander", "ph", "rep"]);
        test_invalid!(app, &["benita-commander", "ph", "rep", "url", "path"]);
        test_invalid!(
            app,
            &[
                "benita-commander",
                "ph",
                "rep",
                "url",
                "path",
                "i2c",
                "extra"
            ]
        );
    }

    #[test]
    fn matching_temperature_valid_responder_subcommands() {
        let mut app = BenitaCommanderApp::new();
        test_valid!(app, &["benita-commander", "temperature"]);
        test_valid!(
            app,
            &[
                "benita-commander",
                "temperature",
                "rep",
                "url",
                "path",
                "i2c"
            ]
        );
    }

    #[test]
    fn matching_temperature_invalid_responder_subcommands_yields_err() {
        let mut app = BenitaCommanderApp::new();
        test_invalid!(app, &["benita-commander", "temperature", "rep"]);
        test_invalid!(
            app,
            &["benita-commander", "temperature", "rep", "url", "path"]
        );
        test_invalid!(
            app,
            &[
                "benita-commander",
                "temperature",
                "rep",
                "url",
                "path",
                "i2c",
                "extra"
            ]
        );
    }
}
