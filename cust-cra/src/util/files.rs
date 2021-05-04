use serde_json::{to_writer_pretty, Value};
use std::fs::{read_to_string, File};

pub fn get_package() -> std::io::Result<Value> {
    // return the contents of package.json as a serde Value object.
    let string = read_to_string("./package.json")?;
    let result = serde_json::from_str(&string)?;
    Ok(result)
}

pub fn save_package(obj: Value) -> std::io::Result<()> {
    // Save modified json object. Overwrites existing package.json.
    let writer = File::create("package.json")?;
    to_writer_pretty(writer, &obj)?;
    Ok(())
}
