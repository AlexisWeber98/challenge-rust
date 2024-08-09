use parse::content_files::parse_env;
use std::io;
use std::io::Error;
use std::{fs::File, io::Write};
// use parse::files::files_to_create::express_env
mod create;
mod parse;

fn create_env(command: &str, project_name: &str) -> Result<(), Error> {
    let mut env = File::create(format!("{}/.env", project_name)).unwrap();
    let mut url = String::new();
    let mut port = String::new();
    let mut server_url = String::new();

    println!("ingrese url de la base de datos mongoDb");
    io::stdin().read_line(&mut url)?;

    if command == "cargo new parse" {
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

        let parse_env_content = parse_env(&url, &app_id, &master_key, &port, &server_url);

        env.write_all(&parse_env_content.as_bytes()).unwrap();
        Ok(())
    } else if command == "cargo new express" {
        /*
        let express_env_content = express_env(&url)?;

        env.write_all(&express_env_content.as_bytes()).unwrap();

        println!("Ingrese el puerto http");
        io::stdin().read_line(&mut port)?;

        println!("Ingrese la url del servidor ( ejemplo: http://localhost:3001 )");
        io::stdin().read_line(&mut server_url)?;

         */

        Ok(())
    } else {
        println!("Comando no válido");
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let mut command = String::new();
    let mut project_name = String::new();

    println!("Ingresa:\n cargo new parse : para crear un nuevo proyecto backend Parse-Server\n cargo new express : para crear un nuevo proyecto express");

    io::stdin().read_line(&mut command)?;
    let command = command.trim();

    println!("Cual es el nombre de tu proyecto?");

    io::stdin().read_line(&mut project_name)?;
    let project_name = project_name.trim();

    create::create(&command, &project_name)?;
    create_env(&command, &project_name)?;

    Ok(())
}
