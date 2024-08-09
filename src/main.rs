use std::fs;
use std::io;
use std::io::Error;
use std::{fs::File, io::Write};

fn create_project() -> Result<(), Error> {
    fs::create_dir("project")?;
    fs::create_dir("project/src")?;
    fs::create_dir("project/clouds")?;
    Ok(())
}

fn create_env(url: &str) -> Result<(), Error>{
    let content = format!("DATABASE_URL: {}", url);
    
    let mut env = File::create("project/.env").unwrap();
    env.write_all(&content.as_bytes()).unwrap();
    
    Ok(())
}

fn create(command: &str) -> Result<(), Error> {
    let mut url = String::new();
    if command == "cargo create project" {
        create_project()?;

        println!("ingrese url de la base de datos mongoDb");
        
        io::stdin().read_line(&mut url)?;
        
        create_env(&url)?;
        
        println!("Recuerde declarar la URL de su base de datos en /project/.env");
        
        // ejecutar cracion de otros package.json
        // ejecitar creacion de index.ts
        // ejecutar creacion de parseServer.ts
        // ejecutar creacion de parseDashboard.ts
        // ejecutar creacion de directorios y archivos 

        Ok(())
    } else {
        println!("no se reconoce el comando");
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let mut command = String::new();
    println!("Ingresa:\n cargo create project\npara crear un nuevo proyecto backend");

    io::stdin().read_line(&mut command)?;
    let command = command.trim();

    create(&command)?;
    
    // println!("Cua es el nombre de tu proyecto?");
    
    
    Ok(())
}
