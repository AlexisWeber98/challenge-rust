use crate::parse::files_create::create_files;
use std::fs;
use std::io::Error;

pub fn create_parse(project_name: &str) -> Result<(), Error> {
    fs::create_dir(format!("{}", project_name))?;
    fs::create_dir(format!("{}/src", project_name))?;
    fs::create_dir(format!("{}/clouds", project_name))?;
    fs::create_dir(format!("{}/clouds/routes", project_name))?;
    fs::create_dir(format!("{}/clouds/modules", project_name))?;
    fs::create_dir(format!("{}/clouds/modules/user", project_name))?;

    create_files(project_name)?;
    Ok(())
}
