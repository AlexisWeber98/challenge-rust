use std::io;
use std::io::Error;
use std::{fs::File, io::Write};

mod create;

fn create_env(project_name: &str, url: &str) -> Result<(), Error> {
    let content = format!("DATABASE_URL: {}", url);

    let mut env = File::create(format!("{}/.env", project_name)).unwrap();
    env.write_all(&content.as_bytes()).unwrap();

    Ok(())
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

    Ok(())
}
