use std::error::Error;
use std::fs;
use std::env;
pub struct Config{
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config{
    pub fn build(args: &[String]) -> Result<Config, &str>{
        if args.len() <3{
            return Err("Must input 2 arguments: query and path")
        }
        let query = args[1].clone();
        let filepath = args[2].clone();
        dbg!();
        let ignore_case:bool;
        if args.len() >=4{
            //Aunque en el environment no se ha tenido en cuenta el valor, aqui si que tenemos en cuenta si se pone true o false
            ignore_case = match args[3].clone().as_str() {
                "1" | "true" => true,
                "0" | "false" => false,
                _ => return Err("Invalid ignore_case argument (3rd), must be set to true(1), false(0) or left empty")
            };
        }else{
            ignore_case= env::var("IGNORE_CASE").is_ok();
        }
        Ok(Config {query, filepath, ignore_case})
    }
}

pub fn run(config: Config)-> Result<(), Box<dyn Error>>{
    let lectura = fs::read_to_string(config.filepath)?;

    let resultado;
    if config.ignore_case {
        resultado = search_case_insensitive(&config.query,&lectura);
    }else{
        resultado = search(&config.query,&lectura);
    }

    println!("Found:");
    if resultado.len() ==0{
        println!("Nothing :(");
        return Ok(())
    }
    for elem in resultado{
        println!("{elem}");
    }

    Ok(())
}

fn search<'a>(query: &str,contents: &'a str)-> Vec<&'a str>{
    let mut resultado: Vec<&str> = Vec::new();

    for linea in contents.lines(){
        if linea.contains(query){
            resultado.push(linea);
        }
    } 

    resultado
}

fn search_case_insensitive<'a>(query: &str,contents: &'a str)-> Vec<&'a str>{
    let mut resultado: Vec<&str> = Vec::new();
    let query = query.to_lowercase();

    for linea in contents.lines(){
        if linea.to_lowercase().contains(&query){
            resultado.push(linea);
        }
    } 

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insens_search_tst(){

        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."],search_case_insensitive(query, contents));
    }

}