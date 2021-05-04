use std::fs::{remove_file, File};
#[allow(dead_code)]
pub fn setup_json(cmd: &str) {
    // create a package.json file for testing with the specified start command.
    let err_message = "An error occured while trying to write test json file";
    let test_json = json!({ "scripts": { "start": cmd }});
    let file = File::create("./package.json").expect(err_message);
    serde_json::to_writer(&file, &test_json).expect(err_message);
}
#[allow(dead_code)]
pub fn cleanup() {
    remove_file("package.json").expect("could not remove package.json");
}
