use text_colorizer::*;
use std::env;
use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}",
                      "Error:".red().bold(),  args.filename, e);
            std::process::exit(1);
        }
    };

    let replace_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace '{}' with '{}': {:?}",
                      "Error:".red().bold(), args.target, args.replacement, e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replace_data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}",
                      "Error:".red().bold(), args.output, e)
        }
    };
}

fn print_usage() {
    eprint!("{} - change occurences of one string into anther", "quickreplace".green());
    eprint!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprint!("{} wrong number of arguments: expected 4, got {}.", "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}


fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}
