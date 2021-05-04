use crate::util::files;
use crate::yarn;
use ansi_term::Color::{Cyan, Green, Red};
use std::process;

pub fn install_cypress() {
    println!("{}", Cyan.paint("Installing Cypress..."));

    // edit package.json as required
    change_package();

    // run yarn commands
    let status = yarn::run_yarn_commands(vec![
        "-D",
        "add",
        "cypress",
        "concurrently",
        "@cypress/instrument-cra",
        "@cypress/code-coverage",
        "nyc",
        "istanbul-lib-coverage",
    ]);
    // exit if yarn command failed
    if !status.success() {
        let message = Red.paint("Yarn install failed for cypress and its peer dependencies");
        eprint!("{}", message);
        process::exit(1)
    }

    let success_message = Green.paint("Cypress installation complete! Remember you still need to change cypress configs for coverage reports to work");
    println!("{}", success_message);
}

fn change_package() {
    // get package.json as an object
    let mut package =
        files::get_package().expect("an error occurred when attempting to open package.json");

    // Get a mutable reference to the scripts object.
    let scripts = package["scripts"]
        .as_object_mut()
        .expect("scripts field not found in json");
    // Add cypress scripts
    scripts.insert(
        "dev".to_string(),
        json!("concurrently -p \"[{name}]\" \"yarn run start*\""),
    );
    scripts.insert("start:cypress".to_string(), json!("cypress open"));

    // get mutable reference to start script
    let start = scripts["start"].as_str().unwrap().to_owned();

    // insert cypress/instrument-cra into start script only if it isn't already there
    if !start.contains("-r @cypress/instrument-cra start") {
        let modified = start.replace("start", "-r @cypress/instrument-cra start");
        scripts.insert("start".to_string(), json!(modified));
    }

    // save modified json.
    files::save_package(package).expect("an error occured while trying to save");
    println!("{}", Cyan.paint("package.json successfully modified!"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_utils;

    #[test]
    #[serial]
    fn test_with_cypress_not_in_start() {
        test_utils::setup_json("react-scripts start");

        change_package();
        let package = files::get_package().unwrap();

        test_utils::cleanup();
        assert_eq!(
            package,
            json!({"scripts": {"start": "react-scripts -r @cypress/instrument-cra start", "dev": "concurrently -p \"[{name}]\" \"yarn run start*\"", "start:cypress": "cypress open"}})
        );
    }
}
