use files_create::create_files;
use std::fs;
use std::io::Error;

use super::files_create;

pub fn create_parse(project_name: &str) -> Result<(), Error> {
    fs::create_dir(format!("{}", project_name))?;
    fs::create_dir(format!("{}/src", project_name))?;
    fs::create_dir(format!("{}/cloud", project_name))?;
    fs::create_dir(format!("{}/cloud/routes", project_name))?;
    fs::create_dir(format!("{}/cloud/modules", project_name))?;
    fs::create_dir(format!("{}/cloud/modules/user", project_name))?;

    create_files(project_name)?;
    Ok(())
}
