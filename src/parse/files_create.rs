use crate::parse::files::files_to_create::{
    config_content, index_content, package_content, parse_server_content,
};

use std::io::Error;
use std::{fs::File, io::Write};

fn package_json(project_name: &str) -> Result<(), Error> {
    let content = package_content();

    let mut package_json = File::create(format!("{}/package.json", project_name)).unwrap();
    package_json.write_all(content.as_bytes())?;

    Ok(())
}

fn index_ts(project_name: &str) -> Result<(), Error> {
    let content = index_content();

    let mut index_ts = File::create(format!("{}/src/index.ts", project_name)).unwrap();
    index_ts.write_all(content.as_bytes())?;

    Ok(())
}

fn config_ts(project_name: &str) -> Result<(), Error> {
    let content = config_content();

    let mut config_ts = File::create(format!("{}/src/config.ts", project_name))?;
    config_ts.write_all(content.as_bytes())?;

    Ok(())
}

fn parse_server_ts(project_name: &str) -> Result<(), Error> {
    let content = parse_server_content();

    let mut parse_server_ts = File::create(format!("{}/src/parseServer.ts", project_name))?;
    parse_server_ts.write_all(content.as_bytes())?;

    Ok(())
}

fn global_d_ts(project_name: &str) -> Result <()>, Error {
    let content = "declare module 'parse-server';"
    
        let mut global_d_ts = File::create(format!("{}/src/global.d.ts", project_name))
            parse_server_ts.write_all(content.as_bytes())?;
    Ok(())
}

pub fn create_files(project_name: &str) -> Result<(), Error> {
    index_ts(project_name)?;
    package_json(project_name)?;
    parse_server_ts(project_name)?;
    config_ts(project_name)?;

    Ok(())
}
