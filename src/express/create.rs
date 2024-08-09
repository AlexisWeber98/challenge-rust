use std::fs;
use std::io::Error;

pub fn create_express(project_name: &str) -> Result<(), Error> {
    fs::create_dir(format!("{}", project_name))?;
    fs::create_dir(format!("{}/src", project_name))?;
    fs::create_dir(format!("{}/src/routes", project_name))?;
    fs::create_dir(format!("{}/src/modules", project_name))?;
    fs::create_dir(format!("{}/src/modules/user", project_name))?;

    Ok(())
}
