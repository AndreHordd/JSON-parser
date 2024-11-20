use anyhow::{anyhow, Result};
use clap::{Arg, Command};
use std::fs;
use JSON_parser::JSONParser;

fn main() -> Result<()> {
    let mut cmd = Command::new("JSON парсер")
        .version("1.0")
        .author("Гордашко Андрій")
        .about("Парсер, який реалізує граматику для JSON файлів")
        .arg(
            Arg::new("file")
                .help("Файл JSON для розбору")
                .required(false)
                .index(1),
        )
        .subcommand(Command::new("authors").about("Показати інформацію про автора"));

    let matches = cmd.clone().get_matches();

    if let Some(_) = matches.subcommand_matches("authors") {
        println!("Автори: Гордашко Андрій");
        return Ok(());
    }
    if let Some(file_path) = matches.get_one::<String>("file") {
        let json_content =
            fs::read_to_string(file_path).map_err(|e| anyhow!("Помилка в читанні файлу: {}", e))?;

        let parsed =
            JSONParser::parse_json(&json_content).map_err(|e| anyhow!("Помилка розбору: {}", e))?;

        for pair in parsed {
            println!("{:#?}", pair);
        }
    } else {
        cmd.print_help()?;
        println!();
    }

    Ok(())
}
