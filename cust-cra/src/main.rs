#[macro_use]
extern crate clap;
use ansi_term::Color::Red;
use clap::{App, AppSettings};
use cust_cra::yarn;
use cust_cra::{cypress, tailwind};
use std::path::Path;
use std::process::exit;
fn check_yarn() {
    // Need to call cmd first on Windows
    let status = yarn::run_yarn_commands(vec!["-v"]);

    if !status.success() {
        eprintln!("{}", Red.paint("could not reach yarn"));
        exit(1)
    }
}

fn main() {
    // check if yarn is reachable, if not, exit.
    check_yarn();

    // check if package.json exists in current directory. If not, exit.
    if !Path::new("./package.json").exists() {
        eprintln!(
            "{}",
            Red.paint(
                "package.json not found in current directory. Make sure to run from project root"
            )
        );
        exit(1)
    }
    // get yaml from current working directory
    let yaml = load_yaml!("cli.yml");

    // create the cli application from the yaml
    let matches = App::from_yaml(yaml)
        .setting(AppSettings::ColorAlways)
        .get_matches();

    for val in matches.values_of("TOOLS").unwrap().collect::<Vec<&str>>() {
        match val {
            "cypress" => cypress::install_cypress(),
            "tailwind" => tailwind::install_tailwind(),
            _ => unreachable!(),
        }
    }
}
