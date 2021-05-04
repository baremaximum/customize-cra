use crate::util::files;
use crate::yarn;
use ansi_term::Color::{Cyan, Green, Red};
use std::{fs, process};

pub fn install_tailwind() {
    println!("{}", Cyan.paint("Installing Tailwind..."));

    // run yarn install commands
    let install_status_one = yarn::run_yarn_commands(vec![
        "-D",
        "add",
        "tailwindcss@npm:@tailwindcss/postcss7-compat",
        "postcss@^7",
        "autoprefixer@^9",
    ]);

    if !install_status_one.success() {
        let message = Red.paint("Failed while installing dev dependencies");
        eprintln!("{}", message);
        process::exit(1);
    }

    let install_status_two = yarn::run_yarn_commands(vec!["add", "@craco/craco"]);

    if !install_status_two.success() {
        let message = Red.paint("Failed while installing dependencies");
        eprintln!("{}", message);
        process::exit(1);
    }
    // make the required changes to package.json
    change_package();

    // create craco config file
    fs::write("craco.config.js", CRACO_CONFIG)
        .expect("An error occured while trying to write craco.config.js");
    // initialize tailwind
    let install_status_three = process::Command::new("cmd")
        .args(&["npx", "tailwindcss", "init"])
        .status()
        .unwrap();

    if !install_status_three.success() {
        let message = Red.paint("Failed while initializing tailwind");
        eprintln!("{}", message);
        process::exit(1);
    }

    // modify tailwind config file
    fs::write("tailwind.config.js", TAILWIND_CONFIG)
        .expect("Failed while attempting to write tailwind config");
    // if you get here, you won!
    let success_message = Green.paint("Tailwind installation complete!");
    println!("{}", success_message);
}

fn change_package() {
    // get package.json as an object
    let mut package =
        files::get_package().expect("an error occurred when attempting to open package.json");

    // Get a reference to the scripts object.
    let scripts = package["scripts"]
        .as_object_mut()
        .expect("scripts field not found in json");

    // clone the keys so you can loop over the clone while modifying the original
    let keys = scripts.keys().cloned().collect::<Vec<String>>();
    for key in keys {
        let script = scripts[&key].as_str().unwrap().to_owned();

        // replace react-scripts with craco, except in the eject script
        if key != "eject" {
            let new = script.replace("react-scripts", "craco");
            scripts.insert(key.to_string(), json!(new));
        }
    }

    // save modified json.
    files::save_package(package).expect("an error occured while trying to save");
    println!("{}", Cyan.paint("package.json successfully modified!"));
}

const CRACO_CONFIG: &str = r#"module.exports = {
    style: {
      postcss: {
        plugins: [
          require('tailwindcss'),
          require('autoprefixer'),
        ],
      },
    },
  }"#;

const TAILWIND_CONFIG: &str = r#" module.exports = {
    purge: [],
 
    purge: ['./src/**/*.{js,jsx,ts,tsx}', './public/index.html'],
     darkMode: false,
     theme: {
       extend: {},
     },
     variants: {
       extend: {},
     },
     plugins: [],
   }"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_utils;

    #[test]
    #[serial]
    fn test_replace_react_scripts() {
        test_utils::setup_json("react-scripts start");

        change_package();
        let package = files::get_package().unwrap();

        test_utils::cleanup();
        assert_eq!(package, json!({"scripts": {"start": "craco start"}}));
    }
}
