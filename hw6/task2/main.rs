mod command;
use command::Command;
use std::str::FromStr;

use std::env;
use std::io::prelude::*;
use std::io;


struct Shell {
    /// Whether the shell should exit its run loop at the next iteration.
    should_exit: bool,
}

impl Shell {
    /// Create a new `Shell`.
    fn new() -> Shell {
        unimplemented!();
    }

    /// Print prompt and wait for input.
    /// Returns None when stdin is closed (Ctrl+D).
    /// Returns input string witout trailing newline.
    fn prompt(&self) -> Option<String> {
        let mut buf = String::new();

        print!("bsys-shell {}$ ", env::current_dir().unwrap().display());
        io::stdout().flush().unwrap();
        let n = io::stdin().read_line(&mut buf).unwrap();

        if n == 0 {
            // EOF.
            None
        } else {
            // snip the trailing newline on an owned String.
            let len = buf.trim_right().len();
            buf.truncate(len);
            Some(buf)
        }
    }
}



fn main() {
    let mut s = Shell::new();
    // now s.loop or whatever ...
}
