use clap::{Arg, ArgMatches, Command};
use figment::Figment;
use pyo3::{prelude::*, types::PyDict};

use crate::Context;

pub(crate) fn build_command() -> Command {
    Command::new("python")
        .about("Interact with Python")
        .subcommand(
            Command::new("eval")
                .about("Evaluate code")
                .arg(Arg::new("code").help("The code to evaluate")),
        )
}

pub(crate) fn process_matches(context: &Context, _config_builder: &Figment, matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("eval") {
        if let Some(code) = matches.get_one::<String>("code") {
            let string = match run_py(code) {
                Ok(value) => value,
                Err(error) => error.to_string(),
            };
            if !context.quiet {
                println!("{:#?}", string);
            }
        }
    }
}

fn run_py(code: &str) -> Result<String, PyErr> {
    Python::with_gil(|py| {
        let locals = PyDict::new(py);
        py.run(code, None, Some(locals))?;
        let output = locals.get_item("output").unwrap();
        Ok(output.to_string())
    })
}
