use rust_challenge::express;
use rust_challenge::parse;
use std::fs::File;
use std::io::{self, Error, Write};

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
    let mut env = File::create(format!("{}/.env", project_name)).unwrap();

    match command {
        "parse" => {
            println!("Ingrese la URL de la base de datos MongoDB (deje vacío para configurar después):");
            let mut url = String::new();
            io::stdin().read_line(&mut url)?;
            let url = url.trim().to_string();

            let env_content = parse::content_files::parse_env(&url, project_name);
            env.write_all(env_content.as_bytes()).unwrap();
        }
        "express" => {
            println!("Ingrese la URL de la base de datos MongoDB (deje vacío para configurar después):");
            let mut url = String::new();
            io::stdin().read_line(&mut url)?;
            let url = url.trim().to_string();

            let env_content = express::content_files::env_content(&url);
            env.write_all(env_content.as_bytes()).unwrap();
        }
        _ => {
            println!("Comando no válido");
        }
    }

    Ok(())
}
