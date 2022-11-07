use clap::{ArgMatches, Command};
use figment::Figment;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

use crate::Context;

pub(crate) fn build_command() -> Command {
    Command::new("python")
        .about("Interact with Python")
        .subcommand(Command::new("sample").about("Run sample"))
}

pub(crate) fn process_matches(_context: &Context, _config_builder: &Figment, matches: &ArgMatches) {
    if let Some(_matches) = matches.subcommand_matches("sample") {
        run_py_sample().unwrap();
    }
}

fn run_py_sample() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval(code, None, Some(locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}
