use express::content_files::env_content;
use parse::content_files::parse_env;
use std::io;
use std::io::Error;
use std::{fs::File, io::Write};
use dialoguer::{theme::ColorfulTheme, Select};

mod create;
mod express;
mod parse;


fn parse_selection(parse_command: &str, project_name: &str)->Result<(),Error> {
    create::create(&parse_command, &project_name)?;
    create_env(&parse_command, &project_name)?;
    Ok(())
}

fn express_selection(express_command: &str, project_name: &str)->Result<(),Error> {
    create::create(&express_command, &project_name)?;
    create_env(&express_command, &project_name)?;
    Ok(())
}

fn create_env(command: &str, project_name: &str) -> Result<(), Error> {
    let mut env = File::create(format!("../{}/.env", project_name)).unwrap();
    let mut url = String::new();
    let mut port = String::new();
    let mut server_url = String::new();

    println!("ingrese url de la base de datos mongoDb");
    io::stdin().read_line(&mut url)?;

    if command == "parse" {
        println!("Ingrese su Application Id");

        let mut app_id = String::new();
        io::stdin().read_line(&mut app_id)?;

        println!("Ingrese su MasterKey");

        let mut master_key = String::new();
        io::stdin().read_line(&mut master_key)?;

        println!("Ingrese el puerto http");
        io::stdin().read_line(&mut port)?;

        println!("Ingrese la url del servidor ( ejemplo: http://localhost:3001 )");
        io::stdin().read_line(&mut server_url)?;

        let parse_env_content = parse_env(&url, &app_id, &master_key, &port, &server_url, &project_name);

        env.write_all(&parse_env_content.as_bytes()).unwrap();
        Ok(())
    } else if command == "express" {
        println!("Ingrese el puerto http");
        io::stdin().read_line(&mut port)?;

        println!("Ingrese la url del servidor ( ejemplo: http://localhost:3001 )");
        io::stdin().read_line(&mut server_url)?;

        let express_env_content = env_content(&url, &port, &server_url);

        env.write_all(&express_env_content.as_bytes()).unwrap();

        Ok(())
    } else {
        println!("Comando no válido");
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let options  = vec!["backend Parse Server", "backend Express"];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Seleccione el tipo de backend que desea montar")
        .items(&options)
        .interact()
        .unwrap();
    
    let parse_command = String::from("parse");
    let express_command = String::from("express");
    let mut project_name = String::new();

    println!("Cual es el nombre de tu proyecto?");

    io::stdin().read_line(&mut project_name)?;
    let project_name = project_name.trim();
    
    match selection {
        0 => parse_selection(&parse_command, project_name)?,
        1 => express_selection(&express_command, &project_name)?,
        _=>println!("Selección inválida"),        
    }


    Ok(())
}
