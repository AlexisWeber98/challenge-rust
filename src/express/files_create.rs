use super::content_files::{config_content, index_content, package_content};

use std::io::Error;
use std::{fs::File, io::Write};

fn package_json(project_name: &str) -> Result<(), Error> {
    let content = package_content();

    let mut package_json = File::create(format!("{}/package.json", project_name))?;
    package_json.write_all(content.as_bytes())?;

    Ok(())
}

fn index_ts(project_name: &str) -> Result<(), Error> {
    let content = index_content();

    let mut index_ts = File::create(format!("{}/src/index.ts", project_name))?;
    index_ts.write_all(content.as_bytes())?;

    Ok(())
}

fn config_ts(project_name: &str) -> Result<(), Error> {
    let content = config_content();

    let mut config_ts = File::create(format!("{}/src/config.ts", project_name))?;
    config_ts.write_all(content.as_bytes())?;

    Ok(())
}

pub fn create_files(project_name: &str) -> Result<(), Error> {
    index_ts(project_name)?;
    package_json(project_name)?;
    config_ts(project_name)?;

    Ok(())
}
