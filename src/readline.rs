//! Command-line readers for user interaction.
use rustyline::error::ReadlineError;
use rustyline::Editor;

mod errors {
    //! Errors and their descriptions
    use rustyline;

    error_chain! {
        errors {
            InvalidArgs {
                description ("invalid command-line arguments")
            }
        }
        foreign_links {
            Readline(rustyline::error::ReadlineError);
        }
    }
}

pub use self::errors::*;

pub enum SensorKind {
    Conductivity,
    Ph,
    Temperature,
}

/// Interactive command line-reader for `benita-commander`
pub enum CommanderReadline {
    Main,
    MainSensor(SensorKind),
    Device(SensorKind),
    Socket(SensorKind),
}

impl CommanderReadline {
    pub fn eval<F>(&self, line: &str, callback: &F) -> Result<String>
    where
        F: Fn(&str) -> String,
    {
        let result = callback(&line);
        Ok(result)
    }

    pub fn start<F>(&self, prompt: &str, callback: F) -> Result<()>
    where
        F: Fn(&str) -> String,
    {
        let mut rl = Editor::<()>::new();
        if let Err(_) = rl.load_history("history.txt") {
            println!("No previous history.");
        }
        loop {
            let readline = rl.readline(prompt);
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_ref());
                    if line == "q" || line == "Q" || line == "exit" || line == "quit" {
                        break;
                    }
                    println!("[.] {}", self.eval(&line, &callback)?);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        rl.save_history("history.txt").unwrap();
        Ok(())
    }
}
