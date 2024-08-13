use rust_challenge::express;
use rust_challenge::parse;

use std::io::Error;

pub fn create(command: &str, project_name: &str) -> Result<(), Error> {
    if command == "parse" {
        parse::create::create_parse(&project_name)?;
        Ok(())
    } else if command == "express" {
        express::create::create_express(&project_name)?;

        Ok(())
    } else {
        println!("no se reconoce el comando");
        Ok(())
    }
}
