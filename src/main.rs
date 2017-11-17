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

#[macro_use]
mod macros;

pub mod cli;
pub mod errors {
    //! Errors and their descriptions
    use super::conductivity;
    use super::logging;
    use super::ph;
    use super::readline;
    use super::temperature;
    use benita;

    error_chain! {
        errors {
            InvalidArgs {
                description ("invalid command-line arguments")
            }
        }
        links {
            Benita(benita::errors::Error, benita::errors::ErrorKind);
            Conductivity(conductivity::Error, conductivity::ErrorKind);
            Logs(logging::Error, logging::ErrorKind);
            Ph(ph::Error, ph::ErrorKind);
            Readline(readline::Error, readline::ErrorKind);
            Temperature(temperature::Error, temperature::ErrorKind);
        }
    }
}

pub mod conductivity;
pub mod ph;
pub mod logging;
pub mod readline;
pub mod temperature;

use std::thread;
use std::time::Duration;

use self::errors::*;
use self::cli::BenitaCommanderApp;
use self::conductivity::{ConductivityREP, ConductivityREQ};
//use self::logging::start_log;
use self::ph::{PhREP, PhREQ};
use self::readline::{CommanderReadline, SensorKind};
use self::temperature::{TemperatureREP, TemperatureREQ};

use benita::network::Endpoint;
use clap::ArgMatches;

/// Execute the program from the given command-line arguments
fn evaluate(args: &ArgMatches) -> Result<()> {
    // This runs the whole program. Everything has brought us to this point.
    // Use it wisely.
    debug!(target: "benita-commander", "{:?}", args);

    let _eval = match args.subcommand() {
        ("", _args) => {
            debug!("top-level readline");
            println!("********************");
            println!("* benita-commander *");
            println!("********************");
            CommanderReadline::Main.start(">> ", |s| s.to_string().to_uppercase())?
        }
        ("conductivity", _args) => {
            debug!("conductivity readline");
            match _args {
                Some(m) => match m.subcommand() {
                    ("rep", Some(rep_args)) => {
                        let url = rep_args.value_of("URL").unwrap();
                        let path = rep_args.value_of("I2C_PATH").unwrap();
                        let addr = rep_args.value_of("I2C_ADDRESS").unwrap().parse().unwrap();

                        let server = ConductivityREP::new(url, path, addr)?;
                        loop {
                            let req_str = &server.responder.recv()?;
                            info!("REQ: {}", &req_str);
                            let call: String = server.eval(&req_str)?;
                            info!("REP: {}", &call);
                            let _reply = &server.responder.send(call.as_bytes())?;

                            thread::sleep(Duration::from_millis(400));
                        }
                    }
                    ("req", Some(req_args)) => {
                        let url = req_args.value_of("URL").unwrap();

                        let requester = ConductivityREQ::new(url)?;

                        let _exec = match req_args.values_of("cmds") {
                            Some(cmds) => for cmd in cmds {
                                let response = match requester.eval(cmd) {
                                    Ok(parsed) => parsed,
                                    _ => "command not in custom api".to_string(),
                                };
                                println!("REP: {}", response);
                            },
                            None => CommanderReadline::MainSensor(SensorKind::Conductivity)
                                .start("conductivity>> ", |s| match requester.eval(s) {
                                    Ok(response) => response,
                                    _ => "command not in custom api".to_string(),
                                })?,
                        };
                    }
                    _ => println!("{}", m.usage()),
                },
                None => {}
            }
        }
        ("ph", _args) => {
            debug!(target: "benita-commander", "ph readline");
            match _args {
                Some(m) => match m.subcommand() {
                    ("rep", Some(rep_args)) => {
                        let url = rep_args.value_of("URL").unwrap();
                        let path = rep_args.value_of("I2C_PATH").unwrap();
                        let addr = rep_args.value_of("I2C_ADDRESS").unwrap().parse().unwrap();

                        let server = PhREP::new(url, path, addr)?;
                        loop {
                            let req_str = &server.responder.recv()?;
                            info!("REQ: {}", &req_str);
                            let call: String = server.eval(&req_str)?;
                            info!("REP: {}", &call);
                            let _reply = &server.responder.send(call.as_bytes())?;

                            thread::sleep(Duration::from_millis(400));
                        }
                    }
                    // REQ Client that connects to the given URL, capable of
                    // interpreting from a list of known commands.
                    ("req", Some(req_args)) => {
                        let url = req_args.value_of("URL").unwrap();

                        let requester = PhREQ::new(url)?;

                        let _exec = match req_args.values_of("cmds") {
                            Some(cmds) => for cmd in cmds {
                                let response = match requester.eval(cmd) {
                                    Ok(parsed) => parsed,
                                    _ => "command not in custom api".to_string(),
                                };
                                println!("REP: {}", response);
                            },
                            None => CommanderReadline::MainSensor(SensorKind::Ph).start(
                                "ph>> ",
                                |s| match requester.eval(s) {
                                    Ok(response) => response,
                                    _ => "command not in custom api".to_string(),
                                },
                            )?,
                        };
                    }
                    _ => println!("{}", m.usage()),
                },
                None => {}
            }
        }
        ("temperature", _args) => {
            debug!(target: "benita-commander", "temperature readline");
            match _args {
                Some(m) => match m.subcommand() {
                    ("rep", Some(rep_args)) => {
                        let url = rep_args.value_of("URL").unwrap();
                        let path = rep_args.value_of("I2C_PATH").unwrap();
                        let addr = rep_args.value_of("I2C_ADDRESS").unwrap().parse().unwrap();

                        let server = TemperatureREP::new(url, path, addr)?;
                        loop {
                            let req_str = &server.responder.recv()?;
                            info!("REQ: {}", &req_str);
                            let call: String = server.eval(&req_str)?;
                            info!("REP: {}", &call);
                            let _reply = &server.responder.send(call.as_bytes())?;

                            thread::sleep(Duration::from_millis(400));
                        }
                    }
                    ("req", Some(req_args)) => {
                        let url = req_args.value_of("URL").unwrap();

                        let requester = TemperatureREQ::new(url)?;

                        let _exec =
                            match req_args.values_of("cmds") {
                                Some(cmds) => for cmd in cmds {
                                    let response = match requester.eval(cmd) {
                                        Ok(parsed) => parsed,
                                        _ => "command not in custom api".to_string(),
                                    };
                                    println!("REP: {}", response);
                                },
                                None => CommanderReadline::MainSensor(SensorKind::Temperature)
                                    .start("ph>> ", |s| match requester.eval(s) {
                                        Ok(response) => response,
                                        _ => "command not in custom api".to_string(),
                                    })?,
                            };
                    }
                    _ => println!("{}", m.usage()),
                },
                None => {}
            }
        }
        _ => println!("{}", args.usage()),
    };

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
        //let _log = start_log()?;

        info!("benita-command starting");
        let _run = evaluate(&matched_args)?;
    }

    warn!("About to stop calibrated-service");
    info!("benita-command stopping");
    Ok(())
}

// fn main() wrapped to handle error chains
quick_main!(run_code);
