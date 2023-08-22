// Handles CLI prompts and arg parsing
// TODO: add cute UI :3

use std::{io::{stdin, stdout, Write}, path::Path, str::FromStr};

/// Struct containing script parameters, collected via CLI at runtime
pub struct Options {
    pub input_path: String,
    pub output_path: String,
    pub k: usize,
    pub iterations: usize,
    pub mode: Mode,
}

/// enum designating how to treat a given CLI arg
enum Prompt {
    Path,
    Safe,
    Num,
    Mode,
}

pub enum Mode {
    Palette,
    Image
}

impl Options {
    pub fn new() -> Options {
        let prompts = [
            "Enter input path: ",
            "Enter output path: ",
            "Enter K value: ",
            "Enter # of iterations: ",
            "Create palette? (y/n): ",
        ];

        let parse = [
            Prompt::Path,
            Prompt::Safe,
            Prompt::Num,
            Prompt::Num,
            Prompt::Mode,
        ];

        // group prompt text andenum together, then call prompt fn
        let opts = prompts
            .iter()
            .zip(parse)
            .map(|(pr, pa)| prompt(pr, pa))
            .collect::<Vec<String>>();

        // return validated result
        Options {
            input_path: opts[0].clone(),
            output_path: opts[1].clone(),
            k: usize::from_str(&opts[2]).unwrap(),
            iterations: usize::from_str(&opts[3]).unwrap(),
            mode: if opts[4] == "y" {Mode::Palette} else {Mode::Image},
        }
    }
}

// Input parsing and validation
fn prompt(line: &str, parse: Prompt) -> String {
    let mut first = true;
    let mut input = String::new();

    let out = stdout();
    let mut handle = out.lock();
    loop {
        // determine whether first attempt
        if first {
            println!("{line}");
        } else {
            handle.write_all(b"invalid input \"").unwrap();
            handle.write_all(input.trim().as_bytes()).unwrap();
            handle.write_all(b"\" please try again \n").unwrap();
            handle.flush().unwrap();
            input.clear();
        }

        stdin().read_line(&mut input).unwrap();

        // Match prompt with enum to determine how its parsed
        // TODO: wrestle with lifetimes, make these return Option<&str> 
        let parse_fn = match parse {
            Prompt::Path => |p: &str| Path::new(&p.trim()).exists() ,
            Prompt::Safe => |_: &str| true,
            Prompt::Num => |n: &str| usize::from_str(n.trim()).is_ok(),
            Prompt::Mode => |m: &str| m.trim() == "y" || m.trim() == "n" ,
        };

        // if parse is succesful, return trimmed line, otherwise prompt again
        if parse_fn(&input) {
            return input.trim().to_string();
        } else {
            first = false;
            continue;
        }
    }
}
