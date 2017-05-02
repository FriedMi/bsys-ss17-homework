//! Command Modul
//!
//! This modul parses a given Input into corresponding Commands
use std::ffi::CString;


#[derive(Debug)]
pub enum Command {
    /// Execute a command with arguments.
    Exec { prog: CString, argv: Vec<CString> },
    /// Empty command.
    Empty,
    /// Exit.
    Exit,
}

/// Trait Method FromStr
impl ::std::str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Command, String> {
        let mut parts = s.split_whitespace();
        let cmd = if let Some(c) = parts.next() {
            c
        } else {
            return Ok(Command::Empty);
        };

        match cmd {
            "exit" => Ok(Command::Exit),
            prog => Command::parse_exec(prog, parts),
        }
    }
}

/// Every given string execpt buildin commands like 'exit' are parsed here to prepare
/// them as an exec() Call later.
impl Command {
    /// Parse an `exec` command.
    fn parse_exec<'a, I>(prog: &str, args: I) -> Result<Command, String>
        where I: Iterator<Item = &'a str>
    {

        let prog = match CString::new(prog) {
            Ok(prog) => prog,
            Err(e) => {
                let error = format!("Null error for prog found: {}", e);
                return Err(error);
            }
        };
        let mut argv = vec![prog];

        for a in args {
            let topush = match CString::new(a) {
                Ok(a) => a,
                Err(e) => {
                    let error = format!("Null error for args found: {}", e);
                    return Err(error);
                }
            };
            argv.push(topush);
        }

        Ok(Command::Exec {
               prog: argv[0].clone(),
               argv: argv,
           })

    }
}
