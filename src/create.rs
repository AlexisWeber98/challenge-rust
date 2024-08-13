use rust_challenge::express;
use rust_challenge::parse;
use std::fs::File;
use std::io::{self, Write, Error};

pub fn create(command: &str, project_name: &str) -> Result<(), Error> {
    match command {
        "parse" => parse::create::create_parse(&project_name),
        "express" => express::create::create_express(&project_name),
        _ => {
            println!("no se reconoce el comando");
            Ok(())
        }
    }
}

pub fn parse_selection(parse_command: &str, project_name: &str) -> Result<(), Error> {
    create(parse_command, project_name)?;
    create_env(parse_command, project_name)?;
    Ok(())
}

pub fn express_selection(express_command: &str, project_name: &str) -> Result<(), Error> {
    create(express_command, project_name)?;
    create_env(express_command, project_name)?;
    Ok(())
}

pub fn create_env(command: &str, project_name: &str) -> Result<(), Error> {
    let mut env = File::create(format!("../{}/.env", project_name)).unwrap();
    let mut url = String::new();
    let mut port = String::new();
    let mut server_url = String::new();

    println!("Ingrese la URL de la base de datos MongoDB");
    io::stdin().read_line(&mut url)?;

    match command {
        "parse" => {
            println!("Ingrese su Application Id");
            let mut app_id = String::new();
            io::stdin().read_line(&mut app_id)?;

            println!("Ingrese su MasterKey");
            let mut master_key = String::new();
            io::stdin().read_line(&mut master_key)?;

            println!("Ingrese el puerto HTTP");
            io::stdin().read_line(&mut port)?;

            println!("Ingrese la URL del servidor (ejemplo: http://localhost:3001)");
            io::stdin().read_line(&mut server_url)?;

            let parse_env_content = parse::content_files::parse_env(&url, &app_id, &master_key, &port, &server_url, project_name);
            env.write_all(parse_env_content.as_bytes()).unwrap();
        }
        "express" => {
            println!("Ingrese el puerto HTTP");
            io::stdin().read_line(&mut port)?;

            println!("Ingrese la URL del servidor (ejemplo: http://localhost:3001)");
            io::stdin().read_line(&mut server_url)?;

            let express_env_content = express::content_files::env_content(&url, &port, &server_url);
            env.write_all(express_env_content.as_bytes()).unwrap();
        }
        _ => {
            println!("Comando no válido");
        }
    }

    Ok(())
}
