use super::content_files::{
    config_content, index_content, package_content, routes_create, user_controllers, user_router,
    user_services,
};

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

    let mut index_ts = File::create(format!("{}/src/index.js", project_name))?;
    index_ts.write_all(content.as_bytes())?;

    Ok(())
}

fn create_user_index_routes(project_name: &str) -> Result<(), Error> {
    let content = user_router();

    let mut user_index = File::create(format!("{}/src/modules/user/userRouter.js", project_name))?;
    user_index.write_all(content.as_bytes())?;

    Ok(())
}

fn create_user_router(project_name: &str) -> Result<(), Error> {
    let content = routes_create();

    let mut index = File::create(format!("{}/src/routes/index.js", project_name))?;
    index.write_all(content.as_bytes())?;
    Ok(())
}

fn create_user_controllers(project_name: &str) -> Result<(), Error> {
    let content = user_controllers();
    let mut user_controllers = File::create(format!(
        "{}/src/modules/user/controllers/userControllers.js",
        project_name
    ))?;
    user_controllers.write_all(content.as_bytes())?;
    Ok(())
}

fn create_user_service(project_name: &str) -> Result<(), Error> {
    let content = user_services();
    let mut user_services = File::create(format!(
        "{}/src/modules/user/services/userServices.js",
        project_name
    ))?;
    user_services.write_all(content.as_bytes())?;

    Ok(())
}

fn config_ts(project_name: &str) -> Result<(), Error> {
    let content = config_content();

    let mut config_ts = File::create(format!("{}/src/config.js", project_name))?;
    config_ts.write_all(content.as_bytes())?;

    Ok(())
}

pub fn create_files(project_name: &str) -> Result<(), Error> {
    index_ts(project_name)?;
    package_json(project_name)?;
    config_ts(project_name)?;
    create_user_index_routes(project_name)?;
    create_user_controllers(project_name)?;
    create_user_service(project_name)?;
    create_user_router(project_name)?;
    Ok(())
}
