mod create;
mod express;
mod parse;

use dialoguer::{theme::ColorfulTheme, Select};
use std::io;
use std::io::Error;

use create::{parse_selection, express_selection};

fn main() -> Result<(), Error> {
    let options = vec!["Backend Parse Server", "Backend Express"];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Seleccione el tipo de backend que desea montar")
        .items(&options)
        .interact()
        .unwrap();
    
    let parse_command = String::from("parse");
    let express_command = String::from("express");
    let mut project_name = String::new();

    println!("¿Cuál es el nombre de tu proyecto?");
    io::stdin().read_line(&mut project_name)?;
    let project_name = project_name.trim();
    
    match selection {
        0 => parse_selection(&parse_command, project_name)?,
        1 => express_selection(&express_command, &project_name)?,
        _ => println!("Selección inválida"),
    }

    Ok(())
}
