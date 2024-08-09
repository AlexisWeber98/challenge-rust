use crate::create_env;
use rust_challenge::parse;
use std::io;
use std::io::Error;

pub fn create(command: &str, project_name: &str) -> Result<(), Error> {
    let mut url = String::new();
    if command == "cargo new parse" {
        parse::create::create_parse(&project_name)?;

        println!("ingrese url de la base de datos mongoDb");

        io::stdin().read_line(&mut url)?;

        create_env(&project_name, &url)?;

        if url.len() > 0 {
        } else {
            println!("Recuerde declarar la URL de su base de datos en /project/.env");
        }

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
