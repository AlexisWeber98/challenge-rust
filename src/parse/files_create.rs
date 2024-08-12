use super::content_files::{
    config_content, index_content, main_content, nodemon_content, package_content,
    parse_server_content, routs_content, tsconfig_content, user_clouds_content, user_controller_content, user_srvices_content,
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

fn global_d_ts(project_name: &str) -> Result<(), Error> {
    let content = String::from("declare module 'parse-server';");

    let mut global_d_ts = File::create(format!("{}/src/global.d.ts", project_name))?;
    global_d_ts.write_all(content.as_bytes())?;

    Ok(())
}

fn tsconfig_json(project_name: &str) -> Result<(), Error> {
    let content = tsconfig_content();

    let mut tsconfig_json = File::create(format!("{}/tsconfig.json", project_name))?;
    tsconfig_json.write_all(content.as_bytes())?;

    Ok(())
}

fn nodemon_json(project_name: &str) -> Result<(), Error> {
    let content = nodemon_content();

    let mut nodemon_json = File::create(format!("{}/nodemon.json", project_name))?;
    nodemon_json.write_all(content.as_bytes())?;

    Ok(())
}

fn main_js(project_name: &str) -> Result<(), Error> {
    let content = main_content();

    let mut main_js = File::create(format!("{}/clouds/main.js", project_name))?;
    main_js.write_all(content.as_bytes())?;

    Ok(())
}

fn index_routs_js(project_name: &str) -> Result<(), Error> {
    let content = routs_content();

    let mut routes_index_js =
        File::create(format!("{}/clouds/routes/index.routes.js", project_name))?;
    routes_index_js.write_all(content.as_bytes())?;

    Ok(())
}

fn user_clouds_js (project_name: &str) -> Result <(), Error> {
    let content = user_clouds_content();
    
    let mut user_clouds_js = File::create(format!("{}/clouds/modules/user/userClouds.js", project_name))?;
    
    user_clouds_js.write_all(content.as_bytes())?;    Ok(())
}

fn user_controller_js(project_name: &str) -> Result<(),Error> {
    let content = user_controller_content();
    
    let mut user_controller_js = File::create(format!("{}/clouds/modules/user/userControllers.js", project_name))?;
    
user_controller_js.write_all(content.as_bytes())?;

Ok(())
}

fn user_service_js (project_name: &str) -> Result<(),Error> {
    let content = user_srvices_content();
    
    let mut user_service_js = File::create(format!("{}/clouds/modules/user/userServices.js", project_name))?;
    
    user_service_js.write_all(content.as_bytes())?;
    
    Ok(())
}

pub fn create_files(project_name: &str) -> Result<(), Error> {
    index_ts(project_name)?;
    package_json(project_name)?;
    parse_server_ts(project_name)?;
    config_ts(project_name)?;
    global_d_ts(project_name)?;
    tsconfig_json(project_name)?;
    nodemon_json(project_name)?;
    main_js(project_name)?;
    index_routs_js(project_name)?;
    user_clouds_js(project_name)?;
    user_controller_js(project_name)?;
    user_service_js(project_name)?;
    
    Ok(())
}
