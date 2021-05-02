#[macro_use]
extern crate clap;
use clap::{App, AppSettings};
use cust_cra::{cypress, tailwind};
use std::process::{exit, Command, Stdio};

fn check_yarn() {
    let status = Command::new("cmd")
        .args(&["/c", "yarn", "-v"])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .unwrap();

    if !status.success() {
        eprintln!("could not reach yarn");
        exit(1)
    }
}

fn main() {
    // check if yarn is reachable, if not, exit
    check_yarn();
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
