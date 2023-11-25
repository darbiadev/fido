//! Running Python

use clap::{Arg, ArgMatches, Command};
use figment::Figment;
use pyo3::{prelude::*, types::PyDict};

use crate::Context;

/// Build the Clap command
pub(crate) fn build_command() -> Command {
    Command::new("python")
        .about("Interact with Python")
        .subcommand(
            Command::new("eval")
                .about("Evaluate code")
                .arg(Arg::new("code").help("The code to evaluate")),
        )
}

/// Process parsed matches and dispatch to functions
pub(crate) fn process_matches(_config_builder: &Figment, matches: &ArgMatches) {
    let context = Context::from_matches(matches);
    if let Some(matches) = matches.subcommand_matches("eval") {
        if let Some(code) = matches.get_one::<String>("code") {
            let string = match run_py(code) {
                Ok(value) => value,
                Err(error) => error.to_string(),
            };
            if !context.quiet {
                println!("{string:#?}");
            }
        }
    }
}

/// Run Python code
fn run_py(code: &str) -> Result<String, PyErr> {
    Python::with_gil(|py| {
        let locals = PyDict::new(py);
        py.run(code, None, Some(locals))?;
        let output = locals.get_item("output").expect("No output");
        Ok(output.expect("Failed to unwrap output").to_string())
    })
}
